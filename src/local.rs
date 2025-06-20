use std::env::current_dir;
use tracing::{info};

mod custom_logging;
use crate::custom_logging::init_logging;

mod args;
use crate::args::Args;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    init_logging();

    let pwd: String = current_dir().unwrap().to_string_lossy().to_string();

    let args: Args = Args::new();
    let command_to_run: String  = args.get_concat_args();

    log_info!(pwd, command_to_run, "Arguments provided");
}
