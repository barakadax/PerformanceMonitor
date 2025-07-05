use crate::avg::LinkedList;
use std::u64;
use sysinfo::{DiskUsage, Pid, ProcessStatus, ProcessesToUpdate, System};

pub async fn disk(pid: Pid) -> ((u64, f64, u64), (u64, f64, u64)) {
    let mut sys: System = System::new_all();
    let mut read_max: u64 = 0;
    let mut read_avg: LinkedList = LinkedList::new();
    let mut read_min: u64 = u64::MAX;
    let mut write_max: u64 = 0;
    let mut write_avg: LinkedList = LinkedList::new();
    let mut write_min: u64 = u64::MAX;

    loop {
        sys.refresh_processes(ProcessesToUpdate::Some(&[pid]), true);

        if let Some(process) = sys.process(pid) {
            let status: ProcessStatus = process.status();

            let disk: DiskUsage = process.disk_usage();

            if read_max < disk.read_bytes {
                read_max = disk.read_bytes;
            }
            read_avg.add(disk.read_bytes as u128);
            if read_min > disk.read_bytes {
                read_min = disk.read_bytes;
            }

            if write_max < disk.written_bytes {
                write_max = disk.written_bytes;
            }
            write_avg.add(disk.written_bytes as u128);
            if write_min > disk.written_bytes {
                write_min = disk.written_bytes;
            }

            if status == ProcessStatus::Zombie {
                break;
            }
        } else {
            break;
        }
    }

    (
        (read_max, read_avg.average(), read_min),
        (write_max, write_avg.average(), write_min),
    )
}
