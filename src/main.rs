use std::fmt::{Debug, Display, Formatter};

trait Drinkable {
    fn consume(&mut self);

    fn get_data(&self) -> String;

    fn stats(&self) {
        println!("{}", self.get_data());
    }
}

#[derive(Debug)]
enum Milk {
    Whole,
    Oat,
    Almond,
}

struct Coffee<T> {
    kind: T,
    milk: Milk,
    ounces: u32,
}

impl<T> Coffee<T> {
    fn new(kind: T, milk: Milk, ounces: u32) -> Self {
        Self { kind, milk, ounces }
    }
}

impl<T: Debug> Debug for Coffee<T> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("COFFEE")
            .field("kind", &self.kind)
            .field("milk", &self.milk)
            .field("ounces", &self.ounces)
            .finish()
    }
}

impl<T: Display> Drinkable for Coffee<T> {
    fn consume(&mut self) {
        self.ounces = 0;
    }

    fn get_data(&self) -> String {
        format!("A delicious {} ounce {}", self.ounces, self.kind)
    }
}

#[derive(Debug)]
struct Soda {
    calories: u32,
    price: f64,
    flavor: String,
    percentage: u32,
}

impl Soda {
    fn new(calories: u32, price: f64, flavor: String) -> Self {
        Self {
            calories,
            price,
            flavor,
            percentage: 100,
        }
    }
}

impl Drinkable for Soda {
    fn consume(&mut self) {
        self.percentage = 0;
    }

    fn get_data(&self) -> String {
        format!("Flavour: {}, Calories: {}", self.flavor, self.calories)
    }
}

impl Display for Soda {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "** {} Soda **", self.flavor)
    }
}

impl Clone for Soda {
    fn clone(&self) -> Self {
        Self {
            calories: self.calories,
            price: self.price,
            flavor: self.flavor.clone(),
            percentage: self.percentage,
        }
    }
}

fn main() {
    let mut coffee = Coffee::<String>::new("Chocolate".to_string(), Milk::Almond, 10);
    let venila_milk_shake = Coffee::<String>::new("Venila".to_string(), Milk::Oat, 12);
    let falluda = Coffee::<String>::new("Falluda".to_string(), Milk::Whole, 30);
    let coke = Soda::new(20, 5.0, "Lemon".to_string());
    let mut pepsi = coke.clone();
    pepsi.flavor = "Mint".to_string();

    coffee.stats();
    coffee.consume();
    venila_milk_shake.stats();
    falluda.stats();

    coke.stats();
    pepsi.stats();
}
