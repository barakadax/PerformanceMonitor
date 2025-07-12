use crate::monitoring::{
    child_process::ChildProcess, cpu::cpu, disk::disk, memory::memory,
    memory_allocation::memory_allocation, thread::threads, virtual_memory::virtual_memory,
};
use std::u64;
use sysinfo::Pid;
use tokio::{join, spawn, task::JoinHandle};
use tracing::debug;

#[derive(serde::Serialize)]
pub struct Monitor {
    pub pid: u32,
    //pub max_memory: u64,
    //pub avg_memory: f64,
    //pub min_memory: u64,
    //pub max_virtual_memory: u64,
    //pub avg_virtual_memory: f64,
    //pub min_virtual_memory: u64,
    pub max_cpu: f32,
    pub avg_cpu: f64,
    pub min_cpu: f32,
    //pub read_max: u64,
    //pub read_avg: f64,
    //pub read_min: u64,
    //pub write_max: u64,
    //pub write_avg: f64,
    //pub write_min: u64,
    //pub stack_max: u64,
    //pub stack_avg: f64,
    //pub stack_min: u64,
    //pub heap_max: u64,
    //pub heap_avg: f64,
    //pub heap_min: u64,
    //pub max_concurrent_threads: u16,
    //pub thread_ids: Vec<u32>,
    pub child_processes: ChildProcess,
}

impl Monitor {
    pub fn monitor_process(pid: u32) -> impl Future<Output = Self> + Send {
        let pid_for_monitor: Pid = Pid::from_u32(pid);

        debug!(child_pid = pid, "pid of child process to monitor");

        async move {
            //let memory_awaitable: JoinHandle<(u64, f64, u64)> = spawn(memory(pid_for_monitor));
            //let virtual_memory_awaitable: JoinHandle<(u64, f64, u64)> =
            //    spawn(virtual_memory(pid_for_monitor));
            let x = cpu(pid_for_monitor);
            let cpu_awaitable: JoinHandle<(f32, f64, f32)> = spawn(cpu(pid_for_monitor));
            //let disk_awaitable: JoinHandle<((u64, f64, u64), (u64, f64, u64))> =
            //    spawn(disk(pid_for_monitor));
            //let memory_allocation_awaitable: JoinHandle<((u64, f64, u64), (u64, f64, u64))> =
            //    spawn(memory_allocation(pid));
            //let threads_awaitable: JoinHandle<(Vec<u32>, u16)> = spawn(threads(pid));
            let child_processes_awaitable: JoinHandle<ChildProcess> =
                spawn(ChildProcess::child_processes(pid));

            let (
                //memory_res,
                //virtual_memory_res,
                cpu_res,
                //disk_res,
                //memory_allocation_res,
                //threads_res,
                child_processes_res,
            ) = join!(
                //memory_awaitable,
                //virtual_memory_awaitable,
                cpu_awaitable,
                //disk_awaitable,
                //memory_allocation_awaitable,
                //threads_awaitable,
                child_processes_awaitable
            );

            //let (max_memory, avg_memory, min_memory) = memory_res.unwrap_or((0, 0.0, 0));
            //let (max_virtual_memory, avg_virtual_memory, min_virtual_memory) =
            //    virtual_memory_res.unwrap_or((0, 0.0, 0));
            let (max_cpu, avg_cpu, min_cpu) = cpu_res.unwrap_or((0.0, 0.0, 0.0));
            //let ((read_max, read_avg, read_min), (write_max, write_avg, write_min)) =
            //    disk_res.unwrap_or(((0, 0.0, 0), (0, 0.0, 0)));
            //let ((stack_max, stack_avg, stack_min), (heap_max, heap_avg, heap_min)) =
            //    memory_allocation_res.unwrap_or(((0, 0.0, 0), (0, 0.0, 0)));
            //let (thread_ids, max_concurrent_threads) = threads_res.unwrap_or((Vec::new(), 0));
            let child_processes: ChildProcess = child_processes_res.unwrap_or(ChildProcess {
                max_concurrent_child_processes: 0,
                child_processes: Vec::new(),
            });

            Monitor {
                pid,
                //max_memory,
                //avg_memory,
                //min_memory,
                //max_virtual_memory,
                //avg_virtual_memory,
                //min_virtual_memory,
                max_cpu,
                avg_cpu,
                min_cpu,
                //read_max,
                //read_avg,
                //read_min,
                //write_max,
                //write_avg,
                //write_min,
                //stack_max,
                //stack_avg,
                //stack_min,
                //heap_max,
                //heap_avg,
                //heap_min,
                //max_concurrent_threads,
                //thread_ids,
                child_processes,
            }
        }
    }
}
