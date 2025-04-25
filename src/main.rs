use std::clone::Clone;

#[derive(Debug)]
struct Appointment {
    doctor: String,
    start_time: String,
    end_time: String,
}

impl Appointment {
    fn new(doctor: &str, start_time: &str, end_time: &str) -> Self {
        Self {
            doctor: doctor.to_string(),
            start_time: start_time.to_string(),
            end_time: end_time.to_string(),
        }
    }
}

impl Clone for Appointment {
    fn clone(&self) -> Self {
        Self {
            doctor: self.doctor.clone(),
            start_time: self.start_time.clone(),
            end_time: self.end_time.clone(),
        }
    }
}

fn main() {
    let morning_app = Appointment::new("Dr.Paul", "7:00 AM", "8:00 AM");
    let mut evening_app = morning_app.clone();

    evening_app.start_time = String::from("7:00 PM");
    evening_app.end_time = String::from("8:00 PM");

    print!("Morning appoinment {:#?}", morning_app);
    println!("\n");
    print!("Evening appoinment {:#?}", evening_app);
}
