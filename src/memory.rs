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