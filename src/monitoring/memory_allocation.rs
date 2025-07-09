use crate::avg::LinkedList;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
    u64,
};
use sysinfo::{Pid, ProcessStatus, ProcessesToUpdate, System};

#[cfg(target_os = "linux")]
pub fn memory_allocation(pid: u32) -> impl Future<Output = ((u64, f64, u64), (u64, f64, u64))> + Send {
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
    }

    let avg_stack_res: f64 = stack_avg.average();
    let avg_heap_res: f64 = heap_avg.average();
    async move {
        (
            (stack_max, avg_stack_res, stack_min),
            (heap_max, avg_heap_res, heap_min),
        )
    }
}

#[cfg(target_os = "macos")]
pub async fn memory_allocation(pid: u32) -> ((u64, f64, u64), (u64, f64, u64)) {
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
pub async fn memory_allocation(pid: u32) -> ((u64, f64, u64), (u64, f64, u64)) {
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
