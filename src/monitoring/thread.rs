use std::{
    collections::HashSet,
    fs::{self},
};
use sysinfo::{Pid, ProcessStatus, ProcessesToUpdate, System};

#[cfg(target_os = "linux")]
pub async fn threads(pid: u32) -> (Vec<u32>, u16) {
    let pid_for_monitor: Pid = Pid::from_u32(pid);
    let mut sys: System = System::new_all();
    let mut threads: HashSet<_> = HashSet::new();
    let path: String = format!("/proc/{}/task", pid);
    let mut max: u16 = 0;

    loop {
        // prevent this loop from blocking,
        // since this loop only exits if the process terminates
        tokio::task::yield_now().await;

        sys.refresh_processes(ProcessesToUpdate::Some(&[pid_for_monitor]), true);
        if let Some(process) = sys.process(pid_for_monitor) {
            if process.status() == ProcessStatus::Zombie {
                break;
            }

            if let Ok(entries) = fs::read_dir(&path) {
                let mut counter: u16 = 0;
                for entry in entries.flatten() {
                    if let Some(name) = entry.file_name().to_str() {
                        if let Ok(tid) = name.parse::<u32>() {
                            counter += 1;
                            threads.insert(tid);
                        }
                    }
                }
                if max < counter {
                    max = counter;
                }
            }
        } else {
            break;
        }
    }

    (threads.into_iter().collect(), max)
}

#[cfg(target_os = "macos")]
pub async fn threads(pid: u32) -> (Vec<u32>, u16) {
    use mach::{
        kern_return::KERN_SUCCESS,
        port::mach_port_t,
        task::{task_for_pid, task_threads},
        traps::mach_task_self,
    };

    let pid_for_monitor: Pid = Pid::from_u32(pid);
    let mut sys: System = System::new_all();
    let mut threads: HashSet<_> = HashSet::new();
    let mut max: u16 = 0;

    loop {
        // prevent this loop from blocking,
        // since this loop only exits if the process terminates
        tokio::task::yield_now().await;

        sys.refresh_processes(ProcessesToUpdate::Some(&[pid_for_monitor]), true);
        if let Some(process) = sys.process(pid_for_monitor) {
            if process.status() == ProcessStatus::Zombie {
                break;
            }

            if let Ok(entries) = fs::read_dir(&path) {
                let mut counter: u16 = 0;
                unsafe {
                    let mut task: mach_port_t = 0;
                    let kr = task_for_pid(mach_task_self(), pid as i32, &mut task);
                    if kr != KERN_SUCCESS {
                        break;
                    }
                    let mut thread_list: *mut mach_port_t = std::ptr::null_mut();
                    let mut thread_count: u32 = 0;
                    let kr = task_threads(task, &mut thread_list, &mut thread_count);
                    if kr != KERN_SUCCESS {
                        break;
                    }
                    for i in 0..thread_count {
                        let tid = *thread_list.add(i as usize);
                        threads.insert(tid as u32);
                        counter += 1;
                    }
                }
                if max < counter {
                    max = counter;
                }
            }
        } else {
            break;
        }
    }

    (threads.into_iter().collect(), max)
}

#[cfg(target_os = "windows")] // I gave up need to learn how to implement later on
pub async fn threads(pid: u32) -> (Vec<u32>, u16) {
    use std::mem::size_of;

    #[cfg(target_pointer_width = "32")]
    use windows::Win32::{
        Foundation::{CloseHandle, HANDLE},
        System::Diagnostics::ToolHelp::{
            CreateToolhelp32Snapshot as CreateTooHelpSnapshot, TH32CS_SNAPTHREAD as SNAPTHREAD,
            THREADENTRY32 as THREADENTRY, Thread32First as ThreadFirst, Thread32Next as ThreadNext,
        },
    };

    #[cfg(target_pointer_width = "64")]
    use windows::Win64::{
        Foundation::{CloseHandle, HANDLE},
        System::Diagnostics::ToolHelp::{
            CreateToolhelp64Snapshot as CreateTooHelpSnapshot, TH64CS_SNAPTHREAD as SNAPTHREAD,
            THREADENTRY64 as THREADENTRY, Thread64First as ThreadFirst, Thread64Next as ThreadNext,
        },
    };

    let pid_for_monitor: Pid = Pid::from_u32(pid);
    let mut sys: System = System::new_all();
    let mut threads: HashSet<_> = HashSet::new();
    let path: String = format!("/proc/{}/task", pid);
    let mut max: u16 = 0;

    (threads.into_iter().collect(), max)
}
