use std::{
    fs::File,
    io::{Error, ErrorKind, Write},
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

#[derive(serde::Serialize)]
pub struct Process {
    pub child_pid: u32,
    pub exit_status: i32,
    pub duration: String,
    pub signal: String,
    pub monitor: Monitor,
}

impl Process {
    pub async fn run_process(args: Args) -> Self {
        let start_time: Instant = Instant::now();
        let child: Child = Self::init_process(args);

        let child_pid: u32 = child.id().unwrap_or(0);

        let monitor_awaitable: JoinHandle<Monitor> = spawn(Monitor::monitor_process(child_pid));

        let child_output: Output = child
            .wait_with_output()
            .await
            .expect("Failed to wait on child");

        let exit_status: i32 = child_output.status.code().unwrap_or_default();

        let res: Process = Process {
            child_pid,
            exit_status,
            duration: format_duration(start_time.elapsed()),
            signal: Self::get_signal(&child_output),
            monitor: monitor_awaitable
                .await
                .expect("Failed to await monitor process"),
        };

        res.save_to_json_file();

        res
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

    fn save_to_json_file(&self) {
        use serde_json::to_string_pretty;
        let json: String = to_string_pretty(self)
            .map_err(|e| Error::new(ErrorKind::Other, e))
            .expect("Object not serializable");
        let mut file: File = File::create("monitor.json").expect("Couldn't save monitor");
        file.write_all(json.as_bytes())
            .expect("Couldn't write monitor json");
        file.flush().expect("Couldn't flush monitor json");
    }
}
