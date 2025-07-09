use crate::avg::LinkedList;
use sysinfo::{Pid, ProcessStatus, ProcessesToUpdate, System};

pub fn cpu(pid: Pid) -> impl Future<Output = (f32, f64, f32)> + Send {
    let mut sys: System = System::new_all();
    let cpu_count: f32 = sys.cpus().len() as f32;
    let mut max: f32 = 0.0;
    let mut avg: LinkedList = LinkedList::new();
    let mut min: f32 = f32::MAX;

    loop {
        sys.refresh_processes(ProcessesToUpdate::Some(&[pid]), true);

        if let Some(process) = sys.process(pid) {
            let status: ProcessStatus = process.status();

            let cpu: f32 = process.cpu_usage() / cpu_count;
            if max < cpu {
                max = cpu;
            }
            avg.add(cpu.ceil() as u128);
            if min > cpu {
                min = cpu;
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
