use tracing::info;

mod custom_logging;
use crate::custom_logging::init_logging;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    init_logging();

    info!("Need to GRPC server here");
}
