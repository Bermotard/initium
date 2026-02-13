//! OS Command Runner Module
//!
//! Provides platform-specific command execution abstraction
//! Supports: execute, execute_with_output, kill_process, is_process_running


/// Output from a command execution
#[derive(Debug, Clone)]
pub struct CommandOutput {
    /// Exit status code
    pub status_code: i32,
    /// Standard output
    pub stdout: String,
    /// Standard error
    pub stderr: String,
    /// Whether execution was successful
    pub success: bool,
}

/// Abstract trait for OS-specific command execution
pub trait OSCommandRunner {
    /// Execute a command without capturing output
    fn execute(&self, program: &str, args: &[&str], timeout_secs: u64) -> Result<(), String>;

    /// Execute a command and capture output
    fn execute_with_output(
        &self,
        program: &str,
        args: &[&str],
        timeout_secs: u64,
    ) -> Result<CommandOutput, String>;

    /// Kill a process by PID
    fn kill_process(&self, pid: u32) -> Result<(), String>;

    /// Check if a process is running by PID
    fn is_process_running(&self, pid: u32) -> bool;
}

/// Linux-specific command runner
#[cfg(target_os = "linux")]
pub struct LinuxCommandRunner;

#[cfg(target_os = "linux")]
impl OSCommandRunner for LinuxCommandRunner {
    fn execute(&self, program: &str, args: &[&str], _timeout_secs: u64) -> Result<(), String> {
        log::info!("Executing (Linux): {} {:?}", program, args);

        let mut cmd = std::process::Command::new(program);
        for arg in args {
            cmd.arg(arg);
        }

        match cmd.output() {
            Ok(output) => {
                if output.status.success() {
                    log::info!("Command executed successfully");
                    Ok(())
                } else {
                    Err(format!("Command failed with status: {}", output.status))
                }
            }
            Err(e) => {
                log::error!("Execution error: {}", e);
                Err(format!("Execution error: {}", e))
            }
        }
    }

    fn execute_with_output(
        &self,
        program: &str,
        args: &[&str],
        _timeout_secs: u64,
    ) -> Result<CommandOutput, String> {
        log::info!("Executing with output (Linux): {} {:?}", program, args);

        let mut cmd = std::process::Command::new(program);
        for arg in args {
            cmd.arg(arg);
        }

        match cmd.output() {
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout).to_string();
                let stderr = String::from_utf8_lossy(&output.stderr).to_string();
                let status_code = output.status.code().unwrap_or(-1);
                let success = output.status.success();

                log::info!("Command completed with status: {}", status_code);

                Ok(CommandOutput {
                    status_code,
                    stdout,
                    stderr,
                    success,
                })
            }
            Err(e) => {
                log::error!("Execution error: {}", e);
                Err(format!("Execution error: {}", e))
            }
        }
    }

    fn kill_process(&self, pid: u32) -> Result<(), String> {
        log::info!("Killing process (Linux): PID {}", pid);

        match std::process::Command::new("kill")
            .args(&["-9", &pid.to_string()])
            .output()
        {
            Ok(output) => {
                if output.status.success() {
                    log::info!("Process killed successfully");
                    Ok(())
                } else {
                    Err(format!("Failed to kill process: {}", output.status))
                }
            }
            Err(e) => {
                log::error!("Kill error: {}", e);
                Err(format!("Kill error: {}", e))
            }
        }
    }

    fn is_process_running(&self, pid: u32) -> bool {
        let proc_path = format!("/proc/{}", pid);
        std::path::Path::new(&proc_path).exists()
    }
}

/// Windows-specific command runner
#[cfg(target_os = "windows")]
pub struct WindowsCommandRunner;

#[cfg(target_os = "windows")]
impl OSCommandRunner for WindowsCommandRunner {
    fn execute(&self, program: &str, args: &[&str], _timeout_secs: u64) -> Result<(), String> {
        log::info!("Executing (Windows): {} {:?}", program, args);

        let mut cmd = std::process::Command::new("cmd");
        cmd.arg("/C").arg(program);
        for arg in args {
            cmd.arg(arg);
        }

        match cmd.output() {
            Ok(output) => {
                if output.status.success() {
                    log::info!("Command executed successfully");
                    Ok(())
                } else {
                    Err(format!("Command failed with status: {}", output.status))
                }
            }
            Err(e) => {
                log::error!("Execution error: {}", e);
                Err(format!("Execution error: {}", e))
            }
        }
    }

    fn execute_with_output(
        &self,
        program: &str,
        args: &[&str],
        _timeout_secs: u64,
    ) -> Result<CommandOutput, String> {
        log::info!("Executing with output (Windows): {} {:?}", program, args);

        let mut cmd = std::process::Command::new("cmd");
        cmd.arg("/C").arg(program);
        for arg in args {
            cmd.arg(arg);
        }

        match cmd.output() {
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout).to_string();
                let stderr = String::from_utf8_lossy(&output.stderr).to_string();
                let status_code = output.status.code().unwrap_or(-1);
                let success = output.status.success();

                log::info!("Command completed with status: {}", status_code);

                Ok(CommandOutput {
                    status_code,
                    stdout,
                    stderr,
                    success,
                })
            }
            Err(e) => {
                log::error!("Execution error: {}", e);
                Err(format!("Execution error: {}", e))
            }
        }
    }

    fn kill_process(&self, pid: u32) -> Result<(), String> {
        log::info!("Killing process (Windows): PID {}", pid);

        match std::process::Command::new("taskkill")
            .args(&["/PID", &pid.to_string(), "/F"])
            .output()
        {
            Ok(output) => {
                if output.status.success() {
                    log::info!("Process killed successfully");
                    Ok(())
                } else {
                    Err(format!("Failed to kill process: {}", output.status))
                }
            }
            Err(e) => {
                log::error!("Kill error: {}", e);
                Err(format!("Kill error: {}", e))
            }
        }
    }

    fn is_process_running(&self, pid: u32) -> bool {
        match std::process::Command::new("tasklist").output() {
            Ok(output) => {
                let output_str = String::from_utf8_lossy(&output.stdout);
                output_str.contains(&pid.to_string())
            }
            Err(_) => false,
        }
    }
}

/// macOS-specific command runner
#[cfg(target_os = "macos")]
pub struct MacOSCommandRunner;

#[cfg(target_os = "macos")]
impl OSCommandRunner for MacOSCommandRunner {
    fn execute(&self, program: &str, args: &[&str], _timeout_secs: u64) -> Result<(), String> {
        log::info!("Executing (macOS): {} {:?}", program, args);

        let mut cmd = std::process::Command::new(program);
        for arg in args {
            cmd.arg(arg);
        }

        match cmd.output() {
            Ok(output) => {
                if output.status.success() {
                    log::info!("Command executed successfully");
                    Ok(())
                } else {
                    Err(format!("Command failed with status: {}", output.status))
                }
            }
            Err(e) => {
                log::error!("Execution error: {}", e);
                Err(format!("Execution error: {}", e))
            }
        }
    }

    fn execute_with_output(
        &self,
        program: &str,
        args: &[&str],
        _timeout_secs: u64,
    ) -> Result<CommandOutput, String> {
        log::info!("Executing with output (macOS): {} {:?}", program, args);

        let mut cmd = std::process::Command::new(program);
        for arg in args {
            cmd.arg(arg);
        }

        match cmd.output() {
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout).to_string();
                let stderr = String::from_utf8_lossy(&output.stderr).to_string();
                let status_code = output.status.code().unwrap_or(-1);
                let success = output.status.success();

                log::info!("Command completed with status: {}", status_code);

                Ok(CommandOutput {
                    status_code,
                    stdout,
                    stderr,
                    success,
                })
            }
            Err(e) => {
                log::error!("Execution error: {}", e);
                Err(format!("Execution error: {}", e))
            }
        }
    }

    fn kill_process(&self, pid: u32) -> Result<(), String> {
        log::info!("Killing process (macOS): PID {}", pid);

        match std::process::Command::new("kill")
            .args(&["-9", &pid.to_string()])
            .output()
        {
            Ok(output) => {
                if output.status.success() {
                    log::info!("Process killed successfully");
                    Ok(())
                } else {
                    Err(format!("Failed to kill process: {}", output.status))
                }
            }
            Err(e) => {
                log::error!("Kill error: {}", e);
                Err(format!("Kill error: {}", e))
            }
        }
    }

    fn is_process_running(&self, pid: u32) -> bool {
        match std::process::Command::new("ps")
            .arg("-p")
            .arg(pid.to_string())
            .output()
        {
            Ok(output) => output.status.success(),
            Err(_) => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_output_creation() {
        let output = CommandOutput {
            status_code: 0,
            stdout: "hello".to_string(),
            stderr: "".to_string(),
            success: true,
        };
        assert_eq!(output.status_code, 0);
        assert_eq!(output.stdout, "hello");
        assert!(output.success);
    }

    #[test]
    fn test_execute_echo() {
        #[cfg(target_os = "linux")]
        let runner = LinuxCommandRunner;
        #[cfg(target_os = "windows")]
        let runner = WindowsCommandRunner;
        #[cfg(target_os = "macos")]
        let runner = MacOSCommandRunner;

        let result = runner.execute("echo", &["test"], 5);
        assert!(result.is_ok());
    }

    #[test]
    fn test_execute_with_output_echo() {
        #[cfg(target_os = "linux")]
        let runner = LinuxCommandRunner;
        #[cfg(target_os = "windows")]
        let runner = WindowsCommandRunner;
        #[cfg(target_os = "macos")]
        let runner = MacOSCommandRunner;

        let result = runner.execute_with_output("echo", &["hello"], 5);
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
        assert!(output.stdout.contains("hello"));
    }

    #[test]
    fn test_execute_with_output_stderr() {
        #[cfg(target_os = "linux")]
        let runner = LinuxCommandRunner;
        #[cfg(target_os = "windows")]
        let runner = WindowsCommandRunner;
        #[cfg(target_os = "macos")]
        let runner = MacOSCommandRunner;

        #[cfg(target_os = "linux")]
        let result = runner.execute_with_output("sh", &["-c", "echo error >&2"], 5);
        #[cfg(target_os = "windows")]
        let result = runner.execute_with_output("cmd", &["/C", "echo error"], 5);
        #[cfg(target_os = "macos")]
        let result = runner.execute_with_output("sh", &["-c", "echo error >&2"], 5);

        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(!output.stderr.is_empty() || !output.stdout.is_empty());
    }

    #[test]
    fn test_is_process_running_current() {
        #[cfg(target_os = "linux")]
        let runner = LinuxCommandRunner;
        #[cfg(target_os = "windows")]
        let runner = WindowsCommandRunner;
        #[cfg(target_os = "macos")]
        let runner = MacOSCommandRunner;

        let pid = std::process::id();
        assert!(runner.is_process_running(pid));
    }

    #[test]
    fn test_is_process_running_fake() {
        #[cfg(target_os = "linux")]
        let runner = LinuxCommandRunner;
        #[cfg(target_os = "windows")]
        let runner = WindowsCommandRunner;
        #[cfg(target_os = "macos")]
        let runner = MacOSCommandRunner;

        // PID 99999 très probablement inexistant
        assert!(!runner.is_process_running(99999));
    }
}
