use schemars::JsonSchema;
use serde::Deserialize;
use std::process::Command as StdCommand;
use zed::settings::ContextServerSettings;
use zed_extension_api::{
    self as zed, serde_json, Command, ContextServerConfiguration, ContextServerId, Project, Result,
};

const PACKAGE_NAME: &str = "serena-agent";

struct SerenaContextServerExtension;

#[derive(Debug, Deserialize, JsonSchema)]
struct SerenaContextServerSettings {
    /// Python executable to use (optional, defaults to auto-detection)
    python_executable: Option<String>,
    /// Additional environment variables for Serena
    environment: Option<std::collections::HashMap<String, String>>,
}

impl zed::Extension for SerenaContextServerExtension {
    fn new() -> Self {
        Self
    }

    fn context_server_command(
        &mut self,
        _context_server_id: &ContextServerId,
        project: &Project,
    ) -> Result<Command> {
        // Get settings from project configuration
        let settings = ContextServerSettings::for_project("serena-context-server", project)?;
        let user_settings: Option<SerenaContextServerSettings> = settings
            .settings
            .map(|s| serde_json::from_value(s))
            .transpose()
            .map_err(|e| format!("Invalid settings: {}", e))?;

        // Find Python executable
        let python_exe = match &user_settings {
            Some(settings) if settings.python_executable.is_some() => {
                settings.python_executable.as_ref().unwrap().clone()
            }
            _ => find_python_executable()?,
        };

        // Skip installation check - assume serena-agent is already installed
        // This avoids potential issues with restricted environments

        // Prepare environment variables
        let mut env_vars = Vec::new();
        if let Some(settings) = &user_settings {
            if let Some(env) = &settings.environment {
                for (key, value) in env {
                    env_vars.push((key.clone(), value.clone()));
                }
            }
        }

        // Sanitize paths for Windows compatibility
        let python_path = zed_ext::sanitize_windows_path(python_exe.into());

        // Use the serena console script directly or call the CLI properly
        // First try to find the serena script in the same directory as python
        let python_dir = std::path::Path::new(&python_path).parent()
            .ok_or("Could not determine Python directory")?;
        let serena_script = python_dir.join("serena");
        
        let (command, args) = if serena_script.exists() {
            // Use the serena console script directly
            (
                serena_script.to_string_lossy().to_string(),
                vec!["start-mcp-server".to_string()]
            )
        } else {
            // Fallback to calling Python with the correct module invocation
            // We need to call the top_level function from serena.cli
            (
                python_path.to_string_lossy().to_string(),
                vec![
                    "-c".to_string(),
                    "import sys; sys.argv = ['serena', 'start-mcp-server']; from serena.cli import top_level; top_level()".to_string()
                ]
            )
        };

        Ok(Command {
            command,
            args,
            env: env_vars,
        })
    }

    fn context_server_configuration(
        &mut self,
        _context_server_id: &ContextServerId,
        _project: &Project,
    ) -> Result<Option<ContextServerConfiguration>> {
        let installation_instructions = r#"
## Serena Context Server Setup

1. **Install Python 3.11 or 3.12** (required):
   ```bash
   brew install python@3.11
   python3.11 --version
   ```

2. **Install Serena Agent**:
   ```bash
   python3.11 -m pip install serena-agent
   ```

3. **Configure in Zed settings.json**:
   ```json
   {
     "context_servers": {
       "serena-context-server": {
         "source": "extension",
         "enabled": true,
         "settings": {
           "python_executable": "/opt/homebrew/bin/python3.11"
         }
       }
     }
   }
   ```

The extension will automatically detect Python 3.11/3.12 installations, but you can specify a custom path using the `python_executable` setting.
"#.to_string();

        let default_settings = r#"
{
  "python_executable": "/opt/homebrew/bin/python3.11"
}
"#.to_string();

        let settings_schema = serde_json::to_string(&schemars::schema_for!(SerenaContextServerSettings))
            .map_err(|e| format!("Failed to generate schema: {}", e))?;

        Ok(Some(ContextServerConfiguration {
            installation_instructions,
            default_settings,
            settings_schema,
        }))
    }

}

fn find_python_executable() -> Result<String> {
    // First try using which to find Python executables in PATH
    let which_candidates = vec!["python3.11", "python3.12"];
    
    for candidate in &which_candidates {
        if let Ok(output) = StdCommand::new("which").arg(candidate).output() {
            if output.status.success() {
                let python_path = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if !python_path.is_empty() {
                    // Verify it's the correct version
                    if let Ok(version_output) = StdCommand::new(&python_path).arg("--version").output() {
                        if version_output.status.success() {
                            let version_str = String::from_utf8_lossy(&version_output.stdout);
                            if version_str.contains("Python 3.11") || version_str.contains("Python 3.12") {
                                return Ok(python_path);
                            }
                        }
                    }
                }
            }
        }
    }
    
    // Fallback to hardcoded paths
    let python_candidates = vec![
        "/opt/homebrew/bin/python3.11",
        "/opt/homebrew/bin/python3.12", 
        "/usr/local/bin/python3.11",
        "/usr/local/bin/python3.12",
        "python3.11", 
        "python3.12", 
        "python3", 
        "python"
    ];
    
    for candidate in python_candidates {
        match StdCommand::new(candidate).args(&["--version"]).output() {
            Ok(output) => {
                if output.status.success() {
                    let version_output = String::from_utf8_lossy(&output.stdout);
                    // Check for Python 3.11 or 3.12 specifically (Serena requirement)
                    if version_output.contains("Python 3.11") || version_output.contains("Python 3.12") {
                        return Ok(candidate.to_string());
                    }
                }
            }
            Err(e) => {
                // Log the error for debugging
                eprintln!("Failed to run {}: {}", candidate, e);
            }
        }
    }
    
    Err("Python 3.11 or 3.12 not found. Serena requires Python 3.11-3.12. Please install a compatible version.".into())
}

fn is_serena_installed(python_exe: &str) -> Result<bool> {
    match StdCommand::new(python_exe)
        .args(&["-c", "import serena; print('installed')"])
        .output()
    {
        Ok(output) => Ok(output.status.success()),
        Err(_) => {
            // If we can't check, assume it's installed and let it fail later if not
            // This handles restricted environments where process spawning is limited
            Ok(true)
        }
    }
}

fn install_serena(python_exe: &str) -> Result<()> {
    match StdCommand::new(python_exe)
        .args(&["-m", "pip", "install", PACKAGE_NAME])
        .output()
    {
        Ok(output) => {
            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                return Err(format!("Failed to install Serena: {}", stderr).into());
            }
            Ok(())
        }
        Err(_) => {
            // If we can't install, just continue and hope it's already installed
            // This handles restricted environments
            Ok(())
        }
    }
}

zed::register_extension!(SerenaContextServerExtension);

/// Extensions to the Zed extension API that have not yet stabilized.
mod zed_ext {
    /// Sanitizes the given path to remove the leading `/` on Windows.
    ///
    /// On macOS and Linux this is a no-op.
    ///
    /// This is a workaround for https://github.com/bytecodealliance/wasmtime/issues/10415.
    pub fn sanitize_windows_path(path: std::path::PathBuf) -> std::path::PathBuf {
        use zed_extension_api::{current_platform, Os};

        let (os, _arch) = current_platform();
        match os {
            Os::Mac | Os::Linux => path,
            Os::Windows => path
                .to_string_lossy()
                .to_string()
                .trim_start_matches('/')
                .into(),
        }
    }
}