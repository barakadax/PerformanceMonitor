use std::{
    borrow::Cow,
    env::consts::OS,
    process::{self, Child, Command, Output, Stdio},
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    thread::sleep,
    time::{Duration, Instant},
};
use sysinfo::{Pid, ProcessStatus, ProcessesToUpdate, System};
use tokio::{spawn, task::JoinHandle};
use tracing::{debug, error, info, warn};

use crate::duration::format_duration;

pub struct Process {
    pub command_to_run: String,
    pub child_pid: u32,
    pub output: Output,
    pub duration: String,
}

impl Process {
    pub async fn run_process(command: &str) -> Self {
        Self::init_process(command);

        let start_time: Instant = Instant::now();
        let child: Child = Self::init_process(command);

        let child_pid: u32 = child.id();

        let force_stop_thread_gracefully: Arc<AtomicBool> = Arc::new(AtomicBool::new(true));
        let force_stop_thread_gracefully_clone: Arc<AtomicBool> =
            Arc::clone(&force_stop_thread_gracefully);
        let last_status_awaitable: JoinHandle<Option<String>> = spawn(Self::pick_at_child_process(
            child_pid,
            force_stop_thread_gracefully_clone,
        ));

        let output: Output = child.wait_with_output().expect("Failed to wait on child");
        force_stop_thread_gracefully.store(false, Ordering::SeqCst);

        let last_status: Option<String> = last_status_awaitable
            .await
            .expect("Failed to await child process status");
        log_debug!(
            child_process_death_status = last_status,
            "Validating child process status"
        );

        let duration: Duration = start_time.elapsed();

        Process {
            command_to_run: command.to_string(),
            child_pid,
            output,
            duration: format_duration(duration),
        }
    }

    fn init_process(command: &str) -> Child {
        let mut terminal: &str = "sh";
        let mut terminal_chain_args: &str = "-c";

        if OS == "windows" {
            terminal = "cmd";
            terminal_chain_args = "/C";
        } else if OS != "macos" && OS != "linux" {
            log_error!("Running command on Unix-like OS: {}", command);
            process::exit(1);
        }

        Command::new(terminal)
            .arg(terminal_chain_args)
            .arg(command)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Failed to spawn command")
    }

    async fn pick_at_child_process(pid: u32, run: Arc<AtomicBool>) -> Option<String> {
        let pid_for_monitor: Pid = Pid::from_u32(pid);
        let mut sys: System = System::new_all();
        let mut last_status: Option<String> = None;

        while run.load(Ordering::SeqCst) {
            sys.refresh_processes(ProcessesToUpdate::Some(&[pid_for_monitor]), true);
            if let Some(process) = sys.process(pid_for_monitor) {
                let status: ProcessStatus = process.status();
                last_status = Some(status.to_string());

                if status == ProcessStatus::Zombie {
                    break;
                }

                sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
            } else {
                break;
            }
        }

        log_debug!(
            graceful_stop_flag = run.load(Ordering::SeqCst),
            "Thread is done"
        );
        last_status
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
