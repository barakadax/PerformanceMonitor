use std::env::current_dir;
use tracing::info;

#[macro_use]
mod custom_logging;
use crate::custom_logging::init_logging;

mod args;
use crate::args::Args;

mod process;
use crate::process::Process;

mod duration;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    init_logging();

    let pwd: String = current_dir().unwrap().to_string_lossy().to_string();

    let args: Args = Args::new();
    let command_to_run: String = args.get_concat_args();

    log_info!(pwd = pwd, input = command_to_run, "Starting");

    let process: Process = Process::run_process(&command_to_run).await;
    process.stdout();
    process.stderr();

    log_info!(
        command_ran = process.command_to_run,
        child_pid = process.child_pid,
        child_duration = process.duration,
        exit_code = process.output.status.code().unwrap_or(-1),
        "Done"
    );
}
