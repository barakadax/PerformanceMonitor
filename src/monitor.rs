use std::thread::sleep;
use sysinfo::{
    DiskUsage, MINIMUM_CPU_UPDATE_INTERVAL, Pid, ProcessStatus, ProcessesToUpdate, System,
};

pub async fn dunno(pid: u32) {
    let pid_for_monitor: Pid = Pid::from_u32(pid);
    let mut sys: System = System::new_all();

    loop {
        sys.refresh_processes(ProcessesToUpdate::Some(&[pid_for_monitor]), true);

        if let Some(process) = sys.process(pid_for_monitor) {
            let status: ProcessStatus = process.status();

            let disk: DiskUsage = process.disk_usage();
            println!("Memory: {} bytes", process.memory());
            println!("Virtual Memory: {} bytes", process.virtual_memory());
            println!("CPU Usage: {:.3}%", process.cpu_usage());
            println!(
                "Disk Usage - read bytes: {}, written bytes: {}",
                disk.read_bytes, disk.written_bytes
            );

            if status == ProcessStatus::Zombie {
                break;
            }

            sleep(MINIMUM_CPU_UPDATE_INTERVAL);
        } else {
            break;
        }
    }
}
