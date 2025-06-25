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

mod monitor;

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
        child_max_memory_in_bytes = process.monitor.max_memory,
        child_avg_memory_in_bytes = process.monitor.avg_memory,
        child_min_memory_in_bytes = process.monitor.min_memory,
        child_max_virtual_memory_in_bytes = process.monitor.max_virtual_memory,
        child_avg_virtual_memory_in_bytes = process.monitor.avg_virtual_memory,
        child_min_virtual_memory_in_bytes = process.monitor.min_virtual_memory,
        child_max_cpu = process.monitor.max_cpu,
        child_avg_cpu = process.monitor.avg_cpu,
        child_min_cpu = process.monitor.min_cpu,
        child_disk_read_max = process.monitor.read_max,
        child_disk_read_avg = process.monitor.read_avg,
        child_disk_read_min = process.monitor.read_min,
        child_disk_write_max = process.monitor.write_max,
        child_disk_write_avg = process.monitor.write_avg,
        child_disk_write_min = process.monitor.write_min,
        "Done"
    );
}
