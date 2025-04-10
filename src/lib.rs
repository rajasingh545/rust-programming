pub mod diet {
    const NUTRITIONIST: &str = "David Smith";

    pub fn ask_about_program() {
        println!("The nutritionist is {}", NUTRITIONIST);
    }
}

pub mod cardio;
pub mod weightlifting;
pub mod file_action;

use cardio::{CardioTool, Exercise as CardioExercise};
use weightlifting::Exercise as WeightliftingExercise;

#[derive(Debug)]
pub struct GymWorkout {
    cardio: CardioExercise,
    weightlifting: WeightliftingExercise,
}

impl GymWorkout {
    pub fn new() -> Self {
        diet::ask_about_program();
        cardio::ask_about_program();
        weightlifting::ask_about_program();

        Self {
            cardio: CardioExercise::new(String::from("Monday"), CardioTool::Treadmill, 30),
            weightlifting: WeightliftingExercise::new(String::from("Monday"), 10),
        }
    }
}
