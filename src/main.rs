use std::fmt::{Debug, Display, Formatter, Result};
use std::fs;
use std::ops::Drop;

enum AppleType {
    RedDelicious,
    GrannySmith,
}

impl Display for AppleType {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        match self {
            AppleType::RedDelicious => write!(formatter, "🍎 Red delicious 🍎"),
            AppleType::GrannySmith => write!(formatter, "🍏 Granny smith 🍏"),
        }
    }
}

impl Debug for AppleType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            AppleType::RedDelicious => write!(f, "AppleType::RedDelicious"),
            AppleType::GrannySmith => write!(f, "AppleType::GrannySmith"),
        }
    }
}

struct Apple {
    kind: AppleType,
    price: f64,
}

impl Display for Apple {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        write!(formatter, "{} for {}", self.kind, self.price)
    }
}

impl Debug for Apple {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        formatter
            .debug_struct("** Apple **")
            .field("Kind", &self.kind)
            .field("price", &self.price)
            .finish()
    }
}

impl Drop for Apple {
    fn drop(&mut self) {
        match fs::remove_file("apple.txt") {
            Ok(_) => println!("Good bye my sweet apple"),
            Err(error) => eprint!("Error in deleting the file: {error}"),
        }
    }
}

fn main() {
    let lunch_snack = Apple {
        kind: AppleType::GrannySmith,
        price: 1.04,
    };

    let delicious_red_apples = Apple {
        kind: AppleType::RedDelicious,
        price: 3.01,
    };

    println!("{}", lunch_snack);
    println!("{}", delicious_red_apples);

    println!("{:?}", lunch_snack);
    println!("{:?}", delicious_red_apples);
}
