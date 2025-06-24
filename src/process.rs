use std::{
    process::{Output, Stdio},
    thread::sleep,
    time::Instant,
};

#[cfg(unix)]
use std::os::unix::process::ExitStatusExt;

use crate::{args::Args, duration::format_duration};
use sysinfo::{MINIMUM_CPU_UPDATE_INTERVAL, Pid, ProcessStatus, ProcessesToUpdate, System};
use tokio::{
    process::{Child, Command},
    spawn,
    task::JoinHandle,
};
use tracing::debug;

pub struct Process {
    pub child_pid: u32,
    pub output: Output,
    pub duration: String,
    pub signal: String,
}

impl Process {
    pub async fn run_process(args: Args) -> Self {
        let start_time: Instant = Instant::now();
        let child: Child = Self::init_process(args);

        let child_pid: u32 = child.id().unwrap_or(0);

        let last_status_awaitable: JoinHandle<Option<String>> =
            spawn(Self::pick_at_child_process(child_pid));

        let child_output: Output = child
            .wait_with_output()
            .await
            .expect("Failed to wait on child");

        let last_status: Option<String> = last_status_awaitable
            .await
            .expect("Failed to await child process status");
        debug!(
            child_process_death_status = last_status,
            "Validating child process status"
        );

        Process {
            child_pid,
            duration: format_duration(start_time.elapsed()),
            signal: Self::get_signal(&child_output),
            output: child_output,
        }
    }

    fn init_process(args: Args) -> Child {
        let mut cmd: Command = Command::new(&args.positional_args[0]);
        for arg in &args.positional_args[1..] {
            cmd.arg(arg);
        }

        cmd.stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
            .expect("Failed to spawn command")
    }

    async fn pick_at_child_process(pid: u32) -> Option<String> {
        let pid_for_monitor: Pid = Pid::from_u32(pid);
        let mut sys: System = System::new_all();
        let mut last_status: Option<String> = None;

        loop {
            sys.refresh_processes(ProcessesToUpdate::Some(&[pid_for_monitor]), true);
            if let Some(process) = sys.process(pid_for_monitor) {
                let status: ProcessStatus = process.status();
                last_status = Some(status.to_string());

                if status == ProcessStatus::Zombie {
                    break;
                }

                sleep(MINIMUM_CPU_UPDATE_INTERVAL);
            } else {
                break;
            }
        }

        last_status
    }

    fn get_signal(output: &Output) -> String {
        #[cfg(unix)]
        {
            format!("{:?}", output.status.signal())
        }
        #[cfg(windows)]
        {
            "unsupported".to_string()
        }
    }
}
