mod file_action;

use file_action::{get_user_input, write_to_file};
use fitness::GymWorkout;

fn main() {
    let workout = GymWorkout::new();

    println!("{:#?}", workout);

    match get_user_input() {
        Ok((file, content)) => {
            write_to_file(&file, &content);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };
}
