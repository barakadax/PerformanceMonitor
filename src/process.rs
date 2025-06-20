use std::{
    borrow::Cow,
    process::{Child, Command, Output},
    time::{Duration, Instant},
};
use tracing::{info, warn};

use crate::duration::format_duration;

pub struct Process {
    pub command_to_run: String,
    pub child_pid: u32,
    pub output: Output,
    pub duration: String,
}

impl Process {
    pub fn run_process(command: &str) -> Self {
        let start_time: Instant = Instant::now();
        let child: Child = Command::new("sh")
            .arg("-c")
            .arg(command)
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .expect("Failed to spawn command");

        let child_pid: u32 = child.id();
        let output: Output = child.wait_with_output().expect("Failed to wait on child");
        let duration: Duration = start_time.elapsed();
        let duration_str: String = format_duration(duration);

        Process {
            command_to_run: command.to_string(),
            child_pid,
            output,
            duration: duration_str,
        }
    }

    pub fn stdout(&self) {
        let stdout: Cow<'_, str> = String::from_utf8_lossy(&self.output.stdout);
        for line in stdout.lines() {
            if !line.trim().is_empty() {
                log_info!(child_message = line.trim(), "stdout from child process");
            }
        }
    }

    pub fn stderr(&self) {
        let stderr: Cow<'_, str> = String::from_utf8_lossy(&self.output.stderr);
        for line in stderr.lines() {
            if !line.trim().is_empty() {
                log_warn!(child_error = line.trim(), "stderr from child process");
            }
        }
    }
}
