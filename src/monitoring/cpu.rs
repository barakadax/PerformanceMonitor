use crate::avg::LinkedList;
use sysinfo::{Pid, ProcessStatus, ProcessesToUpdate, System};

use std::fs::OpenOptions;
use std::io::Write;

pub fn cpu(pid: Pid) -> impl Future<Output = (f32, f64, f32)> + Send {
    let mut sys: System = System::new_all();
    let cpu_count: f32 = sys.cpus().len() as f32;
    let mut max: f32 = 0.0;
    //let mut avg: LinkedList = LinkedList::new().await;
    let mut min: f32 = f32::MAX;

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("a.txt")
        .expect("Failed to open or create file");

    async move {
        loop {
            tokio::task::yield_now().await;

            sys.refresh_processes(ProcessesToUpdate::Some(&[pid]), true);

            if let Some(process) = sys.process(pid) {
                let log_line: String = format!("Monitoring CPU usage for process: {}", pid);
                writeln!(file, "{}", log_line).expect("Failed to write to file");

                let status: ProcessStatus = process.status();

                let cpu: f32 = process.cpu_usage() / cpu_count;
                if max < cpu {
                    max = cpu;
                }
                //avg.add(cpu.ceil() as u128);
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

        (max, 0.0, min)
    }
}
