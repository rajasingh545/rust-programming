fn double_the_length<T>(collection: &Vec<T>) -> usize {
    collection.len() * 2
}

fn last_two<T>(collection: &[T]) -> &[T] {
    let two_from_the_end = collection.len() - 2;
    &collection[two_from_the_end..]
}

fn main() {
    println!("{}", double_the_length(&vec![1, 2, 3]));

    println!("{:?}", last_two(&vec![1, 2, 3, 4, 5, 6]));
}
