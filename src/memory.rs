use std::process::Command;

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
}