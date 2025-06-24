use std::{
    process::{Output, Stdio},
    time::Instant,
};

#[cfg(unix)]
use std::os::unix::process::ExitStatusExt;

use crate::{args::Args, duration::format_duration, monitor::Monitor};
use tokio::{
    process::{Child, Command},
    spawn,
    task::JoinHandle,
};

pub struct Process {
    pub child_pid: u32,
    pub output: Output,
    pub duration: String,
    pub signal: String,
    pub monitor: Monitor,
}

impl Process {
    pub async fn run_process(args: Args) -> Self {
        let start_time: Instant = Instant::now();
        let child: Child = Self::init_process(args);

        let child_pid: u32 = child.id().unwrap_or(0);

        let monitor_awaitable: JoinHandle<Monitor> =
            spawn(Monitor::monitor_process(child_pid));

        let child_output: Output = child
            .wait_with_output()
            .await
            .expect("Failed to wait on child");

        Process {
            child_pid,
            duration: format_duration(start_time.elapsed()),
            signal: Self::get_signal(&child_output),
            output: child_output,
            monitor: monitor_awaitable
                .await
                .expect("Failed to await monitor process"),
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
