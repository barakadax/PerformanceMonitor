use crate::avg::LinkedList;
use std::thread::sleep;
use sysinfo::{
    DiskUsage, MINIMUM_CPU_UPDATE_INTERVAL, Pid, ProcessStatus, ProcessesToUpdate, System,
};
use tokio::{join, spawn, task::JoinHandle};

pub struct Monitor {
    pub max_memory_usage: u64,
    pub avg_memory_usage: f64,
}

impl Monitor {
    pub async fn monitor_process(pid: u32) -> Self {
        let pid_for_monitor: Pid = Pid::from_u32(pid);

        let monitor_memory_awaitable: JoinHandle<u64> = spawn(Self::max_memory(pid_for_monitor));
        let avg_memory_awaitable: JoinHandle<f64> = spawn(Self::avg_memory(pid_for_monitor));

        let (max_memory_usage_res, avg_memory_usage_res) =
            join!(monitor_memory_awaitable, avg_memory_awaitable);

        Monitor {
            max_memory_usage: max_memory_usage_res.unwrap_or(0),
            avg_memory_usage: avg_memory_usage_res.unwrap_or(0.0),
        }
    }

    async fn max_memory(pid: Pid) -> u64 {
        let mut sys: System = System::new_all();
        let mut max: u64 = 0;

        loop {
            sys.refresh_processes(ProcessesToUpdate::Some(&[pid]), true);

            if let Some(process) = sys.process(pid) {
                let status: ProcessStatus = process.status();

                let memory: u64 = process.memory();
                if max < memory {
                    max = memory;
                }

                if status == ProcessStatus::Zombie {
                    break;
                }

                sleep(MINIMUM_CPU_UPDATE_INTERVAL);
            } else {
                break;
            }
        }

        max
    }

    async fn avg_memory(pid: Pid) -> f64 {
        let mut sys: System = System::new_all();
        let mut avg: LinkedList = LinkedList::new();

        loop {
            sys.refresh_processes(ProcessesToUpdate::Some(&[pid]), true);

            if let Some(process) = sys.process(pid) {
                let status: ProcessStatus = process.status();

                avg.add_data(process.memory() as u128);

                if status == ProcessStatus::Zombie {
                    break;
                }

                sleep(MINIMUM_CPU_UPDATE_INTERVAL);
            } else {
                break;
            }
        }

        avg.average()
    }
}

/*pub async fn dunno(pid: u32) {
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
}*/
