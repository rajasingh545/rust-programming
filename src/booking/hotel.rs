use crate::booking::accommodation::Accommodation;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Hotel {
    name: String,
    reservation: HashMap<String, u32>,
}

impl Hotel {
    pub fn new(name: &str) -> Self {
        Hotel {
            name: name.to_string(),
            reservation: HashMap::new(),
        }
    }
}

impl Accommodation for Hotel {
    fn get_description(&self) -> String {
        format!("Hotel: {}", self.name)
    }

    fn book(&mut self, name: &str, nights: u32) -> () {
        self.reservation.insert(name.to_string(), nights);
    }
}
