#[derive(Debug)]
struct TrainSys<'a, 'b> {
    name: &'a str,
    description: &'b str,
}

fn full_name(first_name: &str, last_name: &str) -> String {
    format!("{} {}", first_name, last_name)
}

fn say_hello() -> &'static str {
    "hello"
}

fn main() {
    let train_name = String::from("Vandei Bharat");
    // let system = TrainSys { name: &train_name };

    let mg_train = {
        let description = String::from("Very fast train");
        let travel_plan = TrainSys {
            name: &train_name,
            description: &description,
        };

        travel_plan.name
    };

    println!("Name is {}", full_name("Rajasingh", "Selvakumar"));

    println!("{:?}", say_hello());
    println!("Mg train is {:?}", mg_train);
}
