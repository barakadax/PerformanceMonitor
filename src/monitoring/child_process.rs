use crate::monitor::Monitor;
use futures::future::join_all;
use std::fs::{self};
use sysinfo::{MINIMUM_CPU_UPDATE_INTERVAL, Pid, ProcessStatus, ProcessesToUpdate, System};
use tokio::time::sleep;
use tokio::{spawn, task::JoinHandle};

#[derive(serde::Serialize)]
pub struct ChildProcess {
    pub max_concurrent_child_processes: u16,
    pub child_processes: Vec<Monitor>,
}

impl ChildProcess {
    #[cfg(target_os = "linux")]
    pub fn child_processes(pid: u32) -> impl Future<Output = Self> + Send {
        let mut sys: System = System::new_all();
        let mut max: u16 = 0;
        let mut monitored_pids: Vec<u32> = Vec::new();
        let mut monitor_futures: Vec<JoinHandle<Monitor>> = Vec::new();

        async move {
            loop {
                sleep(MINIMUM_CPU_UPDATE_INTERVAL).await;
                let pid_for_monitor: Pid = Pid::from_u32(pid);
                sys.refresh_processes(ProcessesToUpdate::Some(&[pid_for_monitor]), true);
                if let Some(process) = sys.process(pid_for_monitor) {
                    if process.status() == ProcessStatus::Zombie {
                        break;
                    }
                } else {
                    break;
                }

                if let Ok(entries) = fs::read_dir("/proc") {
                    let mut count: u16 = 0;
                    for entry in entries.flatten() {
                        if let Some(name) = entry.file_name().to_str() {
                            if let Ok(proc_pid) = name.parse::<u32>() {
                                let stat_path: String = format!("/proc/{}/stat", proc_pid);
                                if let Ok(stat) = fs::read_to_string(&stat_path) {
                                    let parts: Vec<&str> = stat.split_whitespace().collect();
                                    if parts.len() > 3 {
                                        if let Ok(ppid) = parts[3].parse::<u32>() {
                                            if ppid == pid && !monitored_pids.contains(&proc_pid) {
                                                count += 1;
                                                monitored_pids.push(proc_pid);
                                                monitor_futures.push(spawn(
                                                    Monitor::monitor_process(proc_pid),
                                                ));
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    if max < count {
                        max = count;
                    }
                }
            }

            let results: Vec<Monitor> = join_all(monitor_futures)
                .await
                .into_iter()
                .filter_map(Result::ok)
                .collect();

            ChildProcess {
                max_concurrent_child_processes: max,
                child_processes: results,
            }
        }
    }
}

#[cfg(target_os = "macos")] // implement sometime
pub async fn child_processes(pid: u32) -> (Vec<u32>, u16) {
    (Vec::new(), 0)
}

#[cfg(target_os = "windows")] // implement sometime
pub async fn child_processes(pid: u32) -> (Vec<u32>, u16) {
    (Vec::new(), 0) // https://learn.microsoft.com/en-us/windows/win32/api/psapi/nf-psapi-enumprocesses
}
