use crate::avg::LinkedList;
use std::{thread::sleep, u64};
use sysinfo::{
    DiskUsage, MINIMUM_CPU_UPDATE_INTERVAL, Pid, ProcessStatus, ProcessesToUpdate, System,
};
use tokio::{join, spawn, task::JoinHandle};

pub struct Monitor {
    pub max_memory: u64,
    pub avg_memory: f64,
    pub min_memory: u64,
    pub max_virtual_memory: u64,
    pub avg_virtual_memory: f64,
    pub min_virtual_memory: u64,
    pub max_cpu: f32,
    pub avg_cpu: f64,
    pub min_cpu: f32,
    pub read_max: u64,
    pub read_avg: f64,
    pub read_min: u64,
    pub write_max: u64,
    pub write_avg: f64,
    pub write_min: u64,
}

impl Monitor {
    pub async fn monitor_process(pid: u32) -> Self {
        let pid_for_monitor: Pid = Pid::from_u32(pid);

        let memory_awaitable: JoinHandle<(u64, f64, u64)> = spawn(Self::memory(pid_for_monitor));
        let virtual_memory_awaitable: JoinHandle<(u64, f64, u64)> =
            spawn(Self::virtual_memory(pid_for_monitor));
        let cpu_awaitable: JoinHandle<(f32, f64, f32)> = spawn(Self::cpu(pid_for_monitor));
        let disk_awaitable: JoinHandle<((u64, f64, u64), (u64, f64, u64))> =
            spawn(Self::disk(pid_for_monitor));

        let (memory_res, virtual_memory_res, cpu_res, disk_res) = join!(
            memory_awaitable,
            virtual_memory_awaitable,
            cpu_awaitable,
            disk_awaitable
        );

        let (max_memory, avg_memory, min_memory) = memory_res.unwrap_or((0, 0.0, 0));
        let (max_virtual_memory, avg_virtual_memory, min_virtual_memory) =
            virtual_memory_res.unwrap_or((0, 0.0, 0));
        let (max_cpu, avg_cpu, min_cpu) = cpu_res.unwrap_or((0.0, 0.0, 0.0));
        let ((read_max, read_avg, read_min), (write_max, write_avg, write_min)) =
            disk_res.unwrap_or(((0, 0.0, 0), (0, 0.0, 0)));

        Monitor {
            max_memory,
            avg_memory,
            min_memory,
            max_virtual_memory,
            avg_virtual_memory,
            min_virtual_memory,
            max_cpu,
            avg_cpu,
            min_cpu,
            read_max,
            read_avg,
            read_min,
            write_max,
            write_avg,
            write_min,
        }
    }

    async fn memory(pid: Pid) -> (u64, f64, u64) {
        let mut sys: System = System::new_all();
        let mut max: u64 = 0;
        let mut avg: LinkedList = LinkedList::new();
        let mut min: u64 = u64::MAX;

        loop {
            sys.refresh_processes(ProcessesToUpdate::Some(&[pid]), true);

            if let Some(process) = sys.process(pid) {
                let status: ProcessStatus = process.status();

                let memory: u64 = process.memory();

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

                sleep(MINIMUM_CPU_UPDATE_INTERVAL);
            } else {
                break;
            }
        }

        (max, avg.average(), min)
    }

    async fn virtual_memory(pid: Pid) -> (u64, f64, u64) {
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

                sleep(MINIMUM_CPU_UPDATE_INTERVAL);
            } else {
                break;
            }
        }

        (max, avg.average(), min)
    }

    async fn cpu(pid: Pid) -> (f32, f64, f32) {
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

                sleep(MINIMUM_CPU_UPDATE_INTERVAL);
            } else {
                break;
            }
        }

        (max, avg.average(), min)
    }

    async fn disk(pid: Pid) -> ((u64, f64, u64), (u64, f64, u64)) {
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

                sleep(MINIMUM_CPU_UPDATE_INTERVAL);
            } else {
                break;
            }
        }

        (
            (read_max, read_avg.average(), read_min),
            (write_max, write_avg.average(), write_min),
        )
    }
}
