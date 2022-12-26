use crate::help;
use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
pub struct SystemProcess {
    pub name: String,
    pub cpu_usage: String,
    pub mem_usage: String,
    pub uptime: String
}