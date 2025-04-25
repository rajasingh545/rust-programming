use rust::booking::accommodation::Accommodation;
use rust::booking::airbnb::AirBnB;
use rust::booking::hotel::Hotel;

fn main() {
    let mut hotel = Hotel::new("Grand Hotel");
    let description = hotel.get_description();
    println!("{}", description);
    hotel.book("Jhon Deo", 3);
    hotel.book("Jane Doe", 5);
    hotel.book("Alice", 2);

    book_for_one_night(&mut hotel, "Bob");
    println!("{:#?}", hotel);

    let mut airbnb = AirBnB::new("Alice");
    let description = airbnb.get_description();
    println!("{}", description);
    airbnb.book("Bob", 2);
    airbnb.book("Charlie", 4);
    airbnb.book("Eve", 3);

    book_for_one_night(&mut airbnb, "Dave");
    println!("{:#?}", airbnb);

    mix_and_match(&hotel, &airbnb);
}

fn book_for_one_night<T: Accommodation>(entity: &mut T, guest: &str) {
    entity.book(guest, 1);
    println!("{} booked for one night", entity.get_description());
}

fn mix_and_match<T: Accommodation, U: Accommodation>(entity1: &T, entity2: &U) {
    println!(
        "Mixing and matching {} and {}",
        entity1.get_description(),
        entity2.get_description()
    );
}
