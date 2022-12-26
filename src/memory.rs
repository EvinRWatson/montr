use std::process::Command;
use rocket::form::validate::Contains;

use serde::{Deserialize, Serialize};
use serde_json::Result;

use crate::help;

#[derive(Serialize, Deserialize)]
pub struct Memory {
    pub capacity: String,
    pub usage: String,
    pub speed: String,
}

impl Default for Memory {
    fn default() -> Self {
        Self {
            capacity: " ".to_string(),
            usage: " ".to_string(),
            speed: " ".to_string()
        }
    }
}

impl Memory {
    pub fn fetch() -> String {
    let output = Command::new("free")
        .arg("-g")
        .arg("-h")
        .arg("-t")
        .output()
        .expect("Failed to execute process");

    let response = String::from_utf8_lossy(&output.stdout);
    print!("{}", response);

    return response.to_string();
    }

    pub fn extract_memory_data(input: String) -> Memory {
        let mut memory: Memory = Memory::default();
        let mut total: String;
        let mut used: String;

        for line in input.lines() {
            match line.to_string() {
                line if line.contains("Mem:") => {
                    let mem_columns: Vec<String> = line.split(" ").map(|s| s.to_string()).collect();
                    total = mem_columns[1].clone();
                    used = mem_columns[2].clone();
                    println!("Total: {}", help::convert_from_gib_to_gb(total));
                    println!("Used: {}", help::convert_from_gib_to_gb(used));
                },
                _ => println!("No match")
            }
            if line.contains("Mem:") {
                total = help::reduce_spaces(input.clone());
            }
        }
        return memory;
    }
}