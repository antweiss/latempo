/// Python executor for dry-run testing
use crate::utils::errors::{Result, WorkflowError};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::Duration;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command as TokioCommand;
use tokio::time::timeout;

/// Result of Python script execution
#[derive(Debug, Clone)]
pub struct ExecutionResult {
    pub success: bool,
    pub stdout: String,
    pub stderr: String,
    pub exit_code: i32,
    pub duration_ms: u64,
}

/// Executor for running Python scripts in isolated environments
pub struct PythonExecutor {
    python_binary: PathBuf,
}

impl PythonExecutor {
    /// Create a new Python executor
    pub fn new() -> Self {
        Self {
            python_binary: PathBuf::from("python3"),
        }
    }

    /// Create a new Python executor with a custom Python binary
    pub fn with_binary(python_binary: PathBuf) -> Self {
        Self { python_binary }
    }

    /// Create a Python virtual environment
    pub async fn create_venv(&self, venv_path: &Path) -> Result<()> {
        tracing::info!("Creating virtual environment at {:?}", venv_path);

        let output = Command::new(&self.python_binary)
            .args(["-m", "venv", venv_path.to_str().unwrap()])
            .output()
            .map_err(|e| {
                WorkflowError::PythonExecutionError(format!("Failed to create venv: {}", e))
            })?;

        if !output.status.success() {
            return Err(WorkflowError::PythonExecutionError(format!(
                "venv creation failed: {}",
                String::from_utf8_lossy(&output.stderr)
            )));
        }

        Ok(())
    }

    /// Install dependencies in a virtual environment
    pub async fn install_dependencies(
        &self,
        venv_path: &Path,
        requirements_file: &Path,
    ) -> Result<()> {
        tracing::info!(
            "Installing dependencies from {:?} in venv {:?}",
            requirements_file,
            venv_path
        );

        let pip_binary = self.get_venv_binary(venv_path, "pip");

        let output = Command::new(pip_binary)
            .args(["install", "-r", requirements_file.to_str().unwrap()])
            .output()
            .map_err(|e| {
                WorkflowError::PythonExecutionError(format!(
                    "Failed to install dependencies: {}",
                    e
                ))
            })?;

        if !output.status.success() {
            return Err(WorkflowError::PythonExecutionError(format!(
                "pip install failed: {}",
                String::from_utf8_lossy(&output.stderr)
            )));
        }

        Ok(())
    }

    /// Execute a Python script with timeout
    pub async fn execute_script(
        &self,
        venv_path: Option<&Path>,
        script_path: &Path,
        timeout_secs: u64,
    ) -> Result<ExecutionResult> {
        tracing::info!(
            "Executing Python script {:?} with timeout {}s",
            script_path,
            timeout_secs
        );

        let python_binary = if let Some(venv) = venv_path {
            self.get_venv_binary(venv, "python")
        } else {
            self.python_binary.clone()
        };

        let start = std::time::Instant::now();

        let mut child = TokioCommand::new(&python_binary)
            .arg(script_path)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| {
                WorkflowError::PythonExecutionError(format!(
                    "Failed to spawn Python process: {}",
                    e
                ))
            })?;

        let stdout = child.stdout.take().ok_or_else(|| {
            WorkflowError::PythonExecutionError("Failed to capture stdout".to_string())
        })?;

        let stderr = child.stderr.take().ok_or_else(|| {
            WorkflowError::PythonExecutionError("Failed to capture stderr".to_string())
        })?;

        // Read stdout and stderr concurrently
        let stdout_task = tokio::spawn(async move {
            let mut reader = BufReader::new(stdout);
            let mut output = String::new();
            let mut line = String::new();
            while reader.read_line(&mut line).await.unwrap_or(0) > 0 {
                output.push_str(&line);
                line.clear();
            }
            output
        });

        let stderr_task = tokio::spawn(async move {
            let mut reader = BufReader::new(stderr);
            let mut output = String::new();
            let mut line = String::new();
            while reader.read_line(&mut line).await.unwrap_or(0) > 0 {
                output.push_str(&line);
                line.clear();
            }
            output
        });

        // Wait for process with timeout
        let wait_result = timeout(Duration::from_secs(timeout_secs), child.wait()).await;

        let (exit_code, success) = match wait_result {
            Ok(Ok(status)) => (status.code().unwrap_or(-1), status.success()),
            Ok(Err(e)) => {
                return Err(WorkflowError::PythonExecutionError(format!(
                    "Failed to wait for process: {}",
                    e
                )));
            }
            Err(_) => {
                // Timeout occurred, kill the process
                let _ = child.kill().await;
                return Err(WorkflowError::PythonExecutionError(format!(
                    "Script execution timed out after {} seconds",
                    timeout_secs
                )));
            }
        };

        let stdout_output = stdout_task.await.unwrap_or_default();
        let stderr_output = stderr_task.await.unwrap_or_default();

        let duration_ms = start.elapsed().as_millis() as u64;

        Ok(ExecutionResult {
            success,
            stdout: stdout_output,
            stderr: stderr_output,
            exit_code,
            duration_ms,
        })
    }

    /// Get the path to a binary in a virtual environment
    fn get_venv_binary(&self, venv_path: &Path, binary_name: &str) -> PathBuf {
        if cfg!(windows) {
            venv_path
                .join("Scripts")
                .join(format!("{}.exe", binary_name))
        } else {
            venv_path.join("bin").join(binary_name)
        }
    }

    /// Check if Python is available
    pub fn check_python_available(&self) -> Result<String> {
        let output = Command::new(&self.python_binary)
            .arg("--version")
            .output()
            .map_err(|e| WorkflowError::PythonExecutionError(format!("Python not found: {}", e)))?;

        if !output.status.success() {
            return Err(WorkflowError::PythonExecutionError(
                "Python check failed".to_string(),
            ));
        }

        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    }
}

impl Default for PythonExecutor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_executor_creation() {
        let executor = PythonExecutor::new();
        assert_eq!(executor.python_binary, PathBuf::from("python3"));
    }

    #[test]
    fn test_executor_with_custom_binary() {
        let executor = PythonExecutor::with_binary(PathBuf::from("/usr/bin/python3.11"));
        assert_eq!(executor.python_binary, PathBuf::from("/usr/bin/python3.11"));
    }

    #[test]
    fn test_check_python_available() {
        let executor = PythonExecutor::new();
        let result = executor.check_python_available();
        assert!(result.is_ok());
        let version = result.unwrap();
        assert!(version.contains("Python"));
    }

    #[tokio::test]
    async fn test_create_venv() {
        let executor = PythonExecutor::new();
        let temp_dir = tempdir().unwrap();
        let venv_path = temp_dir.path().join("test_venv");

        let result = executor.create_venv(&venv_path).await;
        assert!(result.is_ok());
        assert!(venv_path.exists());
    }

    #[tokio::test]
    async fn test_execute_simple_script() {
        let executor = PythonExecutor::new();
        let temp_dir = tempdir().unwrap();
        let script_path = temp_dir.path().join("test_script.py");

        fs::write(&script_path, "print('Hello, World!')").unwrap();

        let result = executor.execute_script(None, &script_path, 5).await;
        assert!(result.is_ok());

        let exec_result = result.unwrap();
        assert!(exec_result.success);
        assert_eq!(exec_result.exit_code, 0);
        assert!(exec_result.stdout.contains("Hello, World!"));
    }

    #[tokio::test]
    async fn test_execute_script_with_error() {
        let executor = PythonExecutor::new();
        let temp_dir = tempdir().unwrap();
        let script_path = temp_dir.path().join("error_script.py");

        fs::write(&script_path, "raise ValueError('Test error')").unwrap();

        let result = executor.execute_script(None, &script_path, 5).await;
        assert!(result.is_ok());

        let exec_result = result.unwrap();
        assert!(!exec_result.success);
        assert_ne!(exec_result.exit_code, 0);
        assert!(exec_result.stderr.contains("ValueError"));
    }

    #[tokio::test]
    async fn test_execute_script_timeout() {
        let executor = PythonExecutor::new();
        let temp_dir = tempdir().unwrap();
        let script_path = temp_dir.path().join("timeout_script.py");

        fs::write(&script_path, "import time; time.sleep(10)").unwrap();

        let result = executor.execute_script(None, &script_path, 1).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("timed out"));
    }
}
