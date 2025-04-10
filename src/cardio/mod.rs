const PERSONAL_TRAINER: &str = "Carl Smith";

/// Define the cardio personal trainer
/// The cardio personal trainer is a constant string that represents the name of the trainer
pub fn ask_about_program() {
    println!("The cardio personal trainer is {}", PERSONAL_TRAINER);
}

/// Define the CardioTool enum to represent different cardio tools
#[derive(Debug)]
pub enum CardioTool {
    Treadmill,
    Elliptical,
    StationaryBike,
}

/// Define the CardioExercise struct to represent a cardio exercise
#[derive(Debug)]
pub struct Exercise {
    day: String,
    tool: CardioTool,
    minutes: u32,
}

impl Exercise {
    pub fn new(day: String, tool: CardioTool, minutes: u32) -> Self {
        Self { day, tool, minutes }
    }
}
