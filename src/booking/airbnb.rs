use crate::booking::accommodation::Accommodation;

#[derive(Debug)]
pub struct AirBnB {
    host: String,
    guests: Vec<(String, u32)>,
}

impl AirBnB {
    pub fn new(host: &str) -> Self {
        Self {
            host: host.to_string(),
            guests: Vec::new(),
        }
    }
}

impl Accommodation for AirBnB {
    fn get_description(&self) -> String {
        format!("AirBnB: {}", self.host)
    }

    fn book(&mut self, name: &str, nights: u32) -> () {
        self.guests.push((name.to_string(), nights));
    }
}
