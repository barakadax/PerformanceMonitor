use std::env::current_dir;
use tracing::info;

mod custom_logging;
use crate::custom_logging::init_logging;

mod args;
use crate::args::Args;

mod process;
use crate::process::Process;

mod duration;

mod monitor;
mod monitoring;

mod avg;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    init_logging();
    //console_subscriber::init();

    let args: Args = Args::new();
    let command_to_run: String = args.get_concat_args();

    info!(
        pwd = current_dir().unwrap().to_string_lossy().to_string(),
        command_to_run, "Starting to run child process"
    );

    Process::run_process(args).await;

    info!("Done");
}
