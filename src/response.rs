use crate::{cpu::Cpu, sys_proc};
use crate::memory::Memory;
use crate::disk::Disk;
use crate::sys_proc::SystemProcess;

use serde::{Deserialize, Serialize};
use serde_json::Result;


#[derive(Serialize, Deserialize)]
pub struct SysData {
    pub cpus: Vec<Cpu>,
    pub memory: Memory,
    pub disks: Disk,
    pub processes: Vec<sys_proc::SystemProcess>
}