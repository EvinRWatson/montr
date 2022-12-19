use std::process::Command;
pub struct Disk {
    pub capacity: String,
    pub usage: String,
    pub speed: String,
    pub smart_status: String
}

impl Default for Disk {
    fn default() -> Self {
        Self {
            capacity: " ".to_string(),
            usage: " ".to_string(),
            speed: " ".to_string(),
            smart_status: " ".to_string()
        }
    }
}

impl Disk {
    pub fn fetch() -> String {
    let output = Command::new("df")
        .arg("-h")
        .output()
        .expect("failed to execute process");

    let response = String::from_utf8_lossy(&output.stdout);
    print!("{}", response);

    return response.to_string();
    }
}
