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

mod avg;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    init_logging();

    let args: Args = Args::new();
    let command_to_run: String = args.get_concat_args();

    let process: Process = Process::run_process(args).await;

    info!(
        pwd = current_dir().unwrap().to_string_lossy().to_string(),
        command_to_run,
        child_pid = process.child_pid,
        child_duration = process.duration,
        child_exit_code = process.output.status.code().unwrap_or(-1),
        child_exit_signal = process.signal,
        "Done"
    );
}
