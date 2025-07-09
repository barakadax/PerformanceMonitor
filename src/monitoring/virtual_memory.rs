use crate::avg::LinkedList;
use std::u64;
use sysinfo::{Pid, ProcessStatus, ProcessesToUpdate, System};

pub fn virtual_memory(pid: Pid) -> impl Future<Output = (u64, f64, u64)> + Send {
    let mut sys: System = System::new_all();
    let mut max: u64 = 0;
    let mut avg: LinkedList = LinkedList::new();
    let mut min: u64 = u64::MAX;

    loop {
        sys.refresh_processes(ProcessesToUpdate::Some(&[pid]), true);

        if let Some(process) = sys.process(pid) {
            let status: ProcessStatus = process.status();

            let memory: u64 = process.virtual_memory();

            if max < memory {
                max = memory;
            }
            avg.add(memory as u128);
            if min > memory {
                min = memory;
            }

            if status == ProcessStatus::Zombie {
                break;
            }
        } else {
            break;
        }
    }

    let avg_res: f64 = avg.average();
    async move { (max, avg_res, min) }
}
