use crate::avg::LinkedList;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
    thread::sleep,
    u64,
};
use sysinfo::{
    DiskUsage, MINIMUM_CPU_UPDATE_INTERVAL, Pid, ProcessStatus, ProcessesToUpdate, System,
};
use tokio::{join, spawn, task::JoinHandle};
use tracing::debug;

#[derive(serde::Serialize)]
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
    pub stack_max: u64,
    pub stack_avg: f64,
    pub stack_min: u64,
    pub heap_max: u64,
    pub heap_avg: f64,
    pub heap_min: u64,
}

impl Monitor {
    pub async fn monitor_process(pid: u32) -> Self {
        let pid_for_monitor: Pid = Pid::from_u32(pid);

        debug!(child_pid = pid, "pid of child process to monitor");

        let memory_awaitable: JoinHandle<(u64, f64, u64)> = spawn(Self::memory(pid_for_monitor));
        let virtual_memory_awaitable: JoinHandle<(u64, f64, u64)> =
            spawn(Self::virtual_memory(pid_for_monitor));
        let cpu_awaitable: JoinHandle<(f32, f64, f32)> = spawn(Self::cpu(pid_for_monitor));
        let disk_awaitable: JoinHandle<((u64, f64, u64), (u64, f64, u64))> =
            spawn(Self::disk(pid_for_monitor));
        let memory_allocation_awaitable: JoinHandle<((u64, f64, u64), (u64, f64, u64))> =
            spawn(Self::memory_allocation(pid));

        let (memory_res, virtual_memory_res, cpu_res, disk_res, memory_allocation_res) = join!(
            memory_awaitable,
            virtual_memory_awaitable,
            cpu_awaitable,
            disk_awaitable,
            memory_allocation_awaitable
        );

        let (max_memory, avg_memory, min_memory) = memory_res.unwrap_or((0, 0.0, 0));
        let (max_virtual_memory, avg_virtual_memory, min_virtual_memory) =
            virtual_memory_res.unwrap_or((0, 0.0, 0));
        let (max_cpu, avg_cpu, min_cpu) = cpu_res.unwrap_or((0.0, 0.0, 0.0));
        let ((read_max, read_avg, read_min), (write_max, write_avg, write_min)) =
            disk_res.unwrap_or(((0, 0.0, 0), (0, 0.0, 0)));
        let ((stack_max, stack_avg, stack_min), (heap_max, heap_avg, heap_min)) =
            memory_allocation_res.unwrap_or(((0, 0.0, 0), (0, 0.0, 0)));

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
            stack_max,
            stack_avg,
            stack_min,
            heap_max,
            heap_avg,
            heap_min,
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

    #[cfg(target_os = "linux")]
    async fn memory_allocation(pid: u32) -> ((u64, f64, u64), (u64, f64, u64)) {
        let pid_for_monitor: Pid = Pid::from_u32(pid);
        let mut sys: System = System::new_all();

        let path: PathBuf = PathBuf::from(format!("/proc/{}/smaps", pid));

        let mut heap_max: u64 = 0;
        let mut heap_avg: LinkedList = LinkedList::new();
        let mut heap_min: u64 = u64::MAX;
        let mut found_heap_flag: bool = false;

        let mut stack_max: u64 = 0;
        let mut stack_avg: LinkedList = LinkedList::new();
        let mut stack_min: u64 = u64::MAX;
        let mut found_stack_flag: bool = false;

        loop {
            sys.refresh_processes(ProcessesToUpdate::Some(&[pid_for_monitor]), true);
            if let Some(process) = sys.process(pid_for_monitor) {
                if process.status() == ProcessStatus::Zombie {
                    break;
                }
            } else {
                break;
            }

            let file: File = match File::open(&path) {
                Ok(f) => f,
                Err(_) => break,
            };
            let reader: BufReader<File> = BufReader::new(file);

            for line in reader.lines().flatten() {
                if line.contains("[heap]") {
                    found_heap_flag = true;
                } else if line.contains("[stack]") {
                    found_stack_flag = true
                }

                if found_heap_flag && line.starts_with("Size:") {
                    if let Some(size_kb) = line.split_whitespace().nth(1) {
                        if let Ok(value) = size_kb.parse::<u64>() {
                            if heap_max < value {
                                heap_max = value;
                            }
                            heap_avg.add(value as u128);
                            if heap_min > value {
                                heap_min = value;
                            }
                        }
                    }
                    found_heap_flag = false;
                } else if found_stack_flag && line.starts_with("Size:") {
                    if let Some(size_kb) = line.split_whitespace().nth(1) {
                        if let Ok(value) = size_kb.parse::<u64>() {
                            if stack_max < value {
                                stack_max = value;
                            }
                            stack_avg.add(value as u128);
                            if stack_min > value {
                                stack_min = value;
                            }
                        }
                    }
                    found_stack_flag = false;
                }
            }

            sleep(MINIMUM_CPU_UPDATE_INTERVAL);
        }

        (
            (stack_max, stack_avg.average(), stack_min),
            (heap_max, heap_avg.average(), heap_min),
        )
    }

    #[cfg(target_os = "macos")]
    async fn memory_allocation(pid: u32) -> ((u64, f64, u64), (u64, f64, u64)) {
        let mut heap_max: u64 = 0;
        let mut heap_avg: LinkedList = LinkedList::new();
        let mut heap_min: u64 = u64::MAX;

        let mut stack_max: u64 = 0;
        let mut stack_avg: LinkedList = LinkedList::new();
        let mut stack_min: u64 = u64::MAX;

        loop {
            unsafe {
                let mut task: task_t = 0;
                if task_for_pid(mach_task_self(), pid as i32, &mut task) != KERN_SUCCESS {
                    break;
                }

                let mut address: mach_vm_address_t = 1;
                let mut size: mach_vm_size_t = 0;
                let mut info = vm_region_basic_info_64::default();
                let mut info_count = VM_REGION_BASIC_INFO_64_COUNT;
                let mut object_name: mach_port_name_t = 0;

                while mach_vm_region(
                    task,
                    &mut address,
                    &mut size,
                    VM_REGION_BASIC_INFO_64,
                    (&mut info as *mut _ as *mut _),
                    &mut info_count,
                    &mut object_name,
                ) == KERN_SUCCESS
                {
                    if info.protection & VM_PROT_READ != 0
                        && info.protection & VM_PROT_WRITE != 0
                        && info.shared == 0
                        && info.is_stack()
                    {
                        if stack_max < size as u64 {
                            stack_max = size as u64;
                        }
                        stack_avg.add(size as u128);
                        if stack_min > size as u64 {
                            stack_min = size as u64;
                        }
                    } else if info.protection & VM_PROT_READ != 0
                        && info.protection & VM_PROT_WRITE != 0
                        && info.shared == 0
                    {
                        if heap_max < size as u64 {
                            heap_max = size as u64;
                        }
                        heap_avg.add(size as u128);
                        if heap_min > size as u64 {
                            heap_min = size as u64;
                        }
                    }
                    address += size;
                }
            }
        }

        (
            (stack_max, stack_avg.average(), stack_min),
            (heap_max, heap_avg.average(), heap_min),
        )
    }

    #[cfg(target_os = "windows")]
    async fn memory_allocation(pid: u32) -> ((u64, f64, u64), (u64, f64, u64)) {
        use std::mem::size_of;

        #[cfg(target_pointer_width = "32")]
        use windows::{
            Win32::Foundation::{CloseHandle, HANDLE},
            Win32::System::Diagnostics::ToolHelp::{
                OpenProcess, PROCESS_QUERY_INFORMATION, PROCESS_VM_READ,
            },
            Win32::System::Memory::{
                MEM_COMMIT, MEM_PRIVATE, MEMORY_BASIC_INFORMATION, PAGE_READWRITE, VirtualQueryEx,
            },
        };

        #[cfg(target_pointer_width = "64")]
        use windows::{
            Win64::Foundation::{CloseHandle, HANDLE},
            Win64::System::Diagnostics::ToolHelp::{
                OpenProcess, PROCESS_QUERY_INFORMATION, PROCESS_VM_READ,
            },
            Win64::System::Memory::{
                MEM_COMMIT, MEM_PRIVATE, MEMORY_BASIC_INFORMATION, PAGE_READWRITE, VirtualQueryEx,
            },
        };

        let mut heap_max: u64 = 0;
        let mut heap_avg: LinkedList = LinkedList::new();
        let mut heap_min: u64 = u64::MAX;

        let mut stack_max: u64 = 0;
        let mut stack_avg: LinkedList = LinkedList::new();
        let mut stack_min: u64 = u64::MAX;

        loop {
            unsafe {
                let process: HANDLE =
                    OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, false, pid);
                if process.is_invalid() {
                    break;
                }

                let mut address = 0usize;
                while address < usize::MAX {
                    let mut mbi = MEMORY_BASIC_INFORMATION::default();
                    let res = VirtualQueryEx(
                        process,
                        address as _,
                        &mut mbi,
                        size_of::<MEMORY_BASIC_INFORMATION>(),
                    );
                    if res == 0 {
                        break;
                    }

                    if mbi.State == MEM_COMMIT
                        && mbi.Protect == PAGE_READWRITE
                        && mbi.Type == MEM_PRIVATE
                    {
                        let region_size = mbi.RegionSize;
                        if region_size >= 0x10000 && region_size <= 0x210000 {
                            if stack_max < region_size as u64 {
                                stack_max = region_size as u64;
                            }
                            stack_avg.add(region_size as u128);
                            if stack_min > region_size as u64 {
                                stack_min = region_size as u64;
                            }
                        } else {
                            if heap_max < region_size as u64 {
                                heap_max = region_size as u64;
                            }
                            heap_avg.add(region_size as u128);
                            if heap_min > region_size as u64 {
                                heap_min = region_size as u64;
                            }
                        }
                    }

                    address = mbi.BaseAddress as usize + mbi.RegionSize;
                }
                CloseHandle(process);
            }
        }

        (
            (stack_max, stack_avg.average(), stack_min),
            (heap_max, heap_avg.average(), heap_min),
        )
    }
}
