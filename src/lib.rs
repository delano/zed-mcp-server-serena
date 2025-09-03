use schemars::JsonSchema;
use serde::Deserialize;
use std::process::Command as StdCommand;
use zed::settings::ContextServerSettings;
use zed_extension_api::{
    self as zed, serde_json, Command, ContextServerConfiguration, ContextServerId, Project, Result,
};

#[allow(dead_code)]
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
            .map(serde_json::from_value)
            .transpose()
            .map_err(|e| format!("Invalid settings: {}", e))?;

        // Find Python executable
        let python_exe = match &user_settings {
            Some(settings) if settings.python_executable.is_some() => settings
                .python_executable
                .as_deref()
                .unwrap_or_default()
                .to_string(),
            _ => find_python_executable()?,
        };

        // Validate the Python executable path for basic security
        if python_exe.is_empty() {
            return Err("Python executable path cannot be empty".into());
        }

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
        let python_dir = std::path::Path::new(&python_path)
            .parent()
            .ok_or("Could not determine Python directory")?;
        let serena_script = python_dir.join("serena");

        let (command, args) = if serena_script.exists() {
            // Use the serena console script directly
            (
                serena_script.to_string_lossy().to_string(),
                vec!["start-mcp-server".to_string()],
            )
        } else {
            // Use proper module invocation instead of inline code manipulation
            (
                python_path.to_string_lossy().to_string(),
                vec![
                    "-m".to_string(),
                    "serena".to_string(),
                    "start-mcp-server".to_string(),
                ],
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

1. **Install Python 3.11 OR 3.12** (either version works):
   ```bash
   # Option A: Install Python 3.11
   brew install python@3.11
   python3.11 --version
   
   # Option B: Install Python 3.12
   brew install python@3.12
   python3.12 --version
   ```

2. **Install Serena Agent** (use the Python version you installed):
   ```bash
   # If you installed Python 3.11:
   python3.11 -m pip install serena-agent
   
   # If you installed Python 3.12:
   python3.12 -m pip install serena-agent
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
  "python_executable": null
}
"#
        .to_string();

        let settings_schema =
            serde_json::to_string(&schemars::schema_for!(SerenaContextServerSettings))
                .map_err(|e| format!("Failed to generate schema: {}", e))?;

        Ok(Some(ContextServerConfiguration {
            installation_instructions,
            default_settings,
            settings_schema,
        }))
    }
}

/// Validates a Python path for security checks
fn validate_python_path(path: &str) -> bool {
    // Enhanced security checks
    if path.is_empty() || path.len() >= 1000 || path.contains('\0') {
        return false;
    }

    // Prevent path traversal attempts
    if path.contains("..") || path.contains("//") {
        return false;
    }

    // Only allow reasonable executable names/paths
    let path_lower = path.to_lowercase();
    path_lower.contains("python")
        || path_lower.starts_with("/usr/")
        || path_lower.starts_with("/opt/")
}

/// Validates Python version string to ensure it's 3.11 or 3.12
fn is_valid_python_version(version_str: &str) -> bool {
    // Use regex-like matching to precisely identify 3.11.x or 3.12.x versions
    let version_str = version_str.trim();

    // Match "Python 3.11" followed by end, space, or dot
    if let Some(rest) = version_str.strip_prefix("Python 3.11") {
        return rest.is_empty() || rest.starts_with('.') || rest.starts_with(' ');
    }

    // Match "Python 3.12" followed by end, space, or dot
    if let Some(rest) = version_str.strip_prefix("Python 3.12") {
        return rest.is_empty() || rest.starts_with('.') || rest.starts_with(' ');
    }

    false
}

fn find_python_executable() -> Result<String> {
    // First try using which to find Python executables in PATH
    let which_candidates = vec!["python3.11", "python3.12"];

    for candidate in &which_candidates {
        if let Ok(output) = StdCommand::new("which").arg(candidate).output() {
            if output.status.success() {
                let python_path = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if !python_path.is_empty() && validate_python_path(&python_path) {
                    // Verify it's the correct version
                    if let Ok(version_output) =
                        StdCommand::new(&python_path).arg("--version").output()
                    {
                        if version_output.status.success() {
                            let version_str = String::from_utf8_lossy(&version_output.stdout);
                            if is_valid_python_version(&version_str) {
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
        "python",
    ];

    for candidate in &python_candidates {
        if !validate_python_path(candidate) {
            continue;
        }

        match StdCommand::new(candidate).args(["--version"]).output() {
            Ok(output) => {
                if output.status.success() {
                    let version_output = String::from_utf8_lossy(&output.stdout);
                    // Check for Python 3.11 or 3.12 specifically (Serena requirement)
                    if is_valid_python_version(&version_output) {
                        return Ok(candidate.to_string());
                    }
                }
            }
            Err(_) => {
                // Skip candidates that can't be executed
                continue;
            }
        }
    }

    let attempted_paths = python_candidates.join(", ");
    Err(format!(
        "Python 3.11 or 3.12 not found in any of these locations: {}. 

Serena requires Python 3.11 OR 3.12 (either version works).

To fix this issue:
1. Install Python 3.11: brew install python@3.11
2. Or install Python 3.12: brew install python@3.12  
3. Or specify custom path in Zed settings: {{\"python_executable\": \"/path/to/python3.11\"}}",
        attempted_paths
    ))
}

#[allow(dead_code)]
fn is_serena_installed(python_exe: &str) -> Result<bool> {
    match StdCommand::new(python_exe)
        .args(["-c", "import serena; print('installed')"])
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

#[allow(dead_code)]
fn install_serena(python_exe: &str) -> Result<()> {
    match StdCommand::new(python_exe)
        .args(["-m", "pip", "install", PACKAGE_NAME])
        .output()
    {
        Ok(output) => {
            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                return Err(format!("Failed to install Serena: {}", stderr));
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

#[cfg(test)]
mod tests {
    use super::*;
    use zed_extension_api::Extension;

    #[test]
    fn test_validate_python_path() {
        // Valid paths
        assert!(validate_python_path("/usr/bin/python3.11"));
        assert!(validate_python_path("/opt/homebrew/bin/python3.12"));
        assert!(validate_python_path("python3.11"));
        assert!(validate_python_path("python3.12"));
        assert!(validate_python_path("python"));

        // Invalid paths
        assert!(!validate_python_path(""));
        assert!(!validate_python_path("path\0with\0null"));
        assert!(!validate_python_path(&"x".repeat(1001))); // Too long
        assert!(!validate_python_path("/etc/../passwd")); // Path traversal
        assert!(!validate_python_path("//malicious//path")); // Double slashes
        assert!(!validate_python_path("malicious-executable")); // Suspicious name
    }

    #[test]
    fn test_is_valid_python_version() {
        // Valid Python 3.11 versions (system needs 3.11 OR 3.12, not both)
        assert!(is_valid_python_version("Python 3.11.0"));
        assert!(is_valid_python_version("Python 3.11.5"));
        assert!(is_valid_python_version(
            "Python 3.11 (default, Oct  5 2023)"
        ));
        assert!(is_valid_python_version("Python 3.11"));
        assert!(is_valid_python_version("  Python 3.11.7  ")); // With whitespace

        // Valid Python 3.12 versions
        assert!(is_valid_python_version("Python 3.12.0"));
        assert!(is_valid_python_version("Python 3.12.1"));
        assert!(is_valid_python_version("Python 3.12 (main, Dec  7 2023)"));

        // Invalid versions - should NOT match
        assert!(!is_valid_python_version("Python 3.10.0"));
        assert!(!is_valid_python_version("Python 3.13.0"));
        assert!(!is_valid_python_version("Python 2.7.0"));
        assert!(!is_valid_python_version("Python 3.9.0"));
        assert!(!is_valid_python_version("Python 3.110.0")); // Edge case - should not match
        assert!(!is_valid_python_version("Python 3.120.0")); // Edge case - should not match
        assert!(!is_valid_python_version("Some Python 3.11.0 thing")); // Doesn't start with "Python 3.11"
    }

    #[test]
    fn test_extension_initialization() {
        let _extension = SerenaContextServerExtension::new();
        // Extension should initialize without panicking
    }

    #[test]
    fn test_serena_context_server_settings_deserialization() {
        // Test valid JSON settings
        let json_str = r#"
        {
            "python_executable": "/usr/bin/python3.11",
            "environment": {
                "SERENA_LOG_LEVEL": "debug"
            }
        }
        "#;

        let settings: Result<SerenaContextServerSettings, _> = serde_json::from_str(json_str);
        assert!(settings.is_ok());

        let settings = settings.unwrap();
        assert_eq!(
            settings.python_executable,
            Some("/usr/bin/python3.11".to_string())
        );
        assert!(settings.environment.is_some());

        // Test minimal valid JSON
        let minimal_json = r#"{}"#;
        let minimal_settings: Result<SerenaContextServerSettings, _> =
            serde_json::from_str(minimal_json);
        assert!(minimal_settings.is_ok());
    }

    #[test]
    fn test_package_name_constant() {
        assert_eq!(PACKAGE_NAME, "serena-agent");
    }
}
