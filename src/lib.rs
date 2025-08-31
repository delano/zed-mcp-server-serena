use schemars::JsonSchema;
use serde::Deserialize;
use std::process::Command as StdCommand;
use zed::settings::ContextServerSettings;
use zed_extension_api::{
    self as zed, serde_json, Command, ContextServerId, Project, Result,
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
        let settings = ContextServerSettings::for_project("serena-context-server", project).ok();
        let user_settings: Option<SerenaContextServerSettings> = settings
            .and_then(|s| s.settings)
            .and_then(|s| serde_json::from_value(s).ok());

        // Find Python executable
        let python_exe = match &user_settings {
            Some(settings) if settings.python_executable.is_some() => {
                settings.python_executable.as_ref().unwrap().clone()
            }
            _ => find_python_executable()?,
        };

        // Check if serena-agent is installed
        if !is_serena_installed(&python_exe)? {
            install_serena(&python_exe)?;
        }

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

        Ok(Command {
            command: python_path.to_string_lossy().to_string(),
            args: vec!["-m".to_string(), "serena.cli".to_string(), "start_mcp_server".to_string()],
            env: env_vars,
        })
    }

}

fn find_python_executable() -> Result<String> {
    // List of Python executables to try, in order of preference
    let python_candidates = vec!["python3.11", "python3.12", "python3", "python"];
    
    for candidate in python_candidates {
        if let Ok(output) = StdCommand::new(candidate)
            .args(&["--version"])
            .output()
        {
            if output.status.success() {
                let version_output = String::from_utf8_lossy(&output.stdout);
                if version_output.contains("Python 3.1") {
                    return Ok(candidate.to_string());
                }
            }
        }
    }
    
    Err("Python 3.11 or later not found. Please install Python 3.11+ and ensure it's in your PATH.".into())
}

fn is_serena_installed(python_exe: &str) -> Result<bool> {
    let output = StdCommand::new(python_exe)
        .args(&["-c", "import serena; print('installed')"])
        .output()
        .map_err(|e| format!("Failed to check Serena installation: {}", e))?;
    
    Ok(output.status.success())
}

fn install_serena(python_exe: &str) -> Result<()> {
    let output = StdCommand::new(python_exe)
        .args(&["-m", "pip", "install", PACKAGE_NAME])
        .output()
        .map_err(|e| format!("Failed to install Serena: {}", e))?;
    
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Failed to install Serena: {}", stderr).into());
    }
    
    Ok(())
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