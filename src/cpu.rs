use std::process::Command;

pub struct Cpu {
    usage: String,
    tempurature: String,
    cores: String,
}

impl Default for Cpu {
    fn default() -> Self {
        Self {
            usage: " ".to_string(),
            tempurature: " ".to_string(),
            cores: " ".to_string()
        }
    }
}

impl Cpu {
    pub fn fetch() -> String {
    let output = Command::new("lscpu")
        .output()
        .expect("Failed to execute process");

    let response = String::from_utf8_lossy(&output.stdout);
    print!("{}", response);

    return response.to_string();
    }
}