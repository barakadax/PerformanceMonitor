use tracing::{debug, error, info, trace, warn};

mod custom_logging;
use crate::custom_logging::init_logging;

mod args;
use crate::args::Args;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    init_logging();

    log_trace!("This is service runtime!");

    let args: Args = Args::new();

    if args.positional_args.is_empty() {
        log_warn!("No arguments provided.");
    } else {
        let concat: String  = args.get_concat_args();
        log_info!(concat, "Arguments provided");

        for (index, arg) in args.positional_args.iter().enumerate() {
            log_debug!("Argument {}: \"{}\"", index, arg);
        }
    }

    log_error!("This is an example error log.");
}
