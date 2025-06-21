use std::{
    borrow::Cow,
    process::{Child, Command, Output},
    thread::sleep,
    time::{Duration, Instant},
};
use sysinfo::{Pid, ProcessStatus, ProcessesToUpdate, System};
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

        let run: bool = true;
        Self::pick_at_child_process(child_pid, run);

        let output: Output = child.wait_with_output().expect("Failed to wait on child");

        let duration: Duration = start_time.elapsed();

        Process {
            command_to_run: command.to_string(),
            child_pid,
            output,
            duration: format_duration(duration),
        }
    }

    fn pick_at_child_process(pid: u32, run: bool) {
        let pid_for_monitor: Pid = Pid::from_u32(pid);
        let mut sys: System = System::new_all();

        while run {
            sys.refresh_processes(ProcessesToUpdate::Some(&[pid_for_monitor]), true);
            if let Some(process) = sys.process(pid_for_monitor) {
                let status: ProcessStatus = process.status();
                log_info!(
                    child_process_status = status.to_string(),
                    "Child process status"
                );

                sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
            } else {
                break;
            }
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
