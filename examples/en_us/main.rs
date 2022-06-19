use mock_rs::{Person, PersonTrait};

fn main() {
    let person = Person::new();

    let first_name = person.first_name(None);
    println!("{}", first_name);

    let last_name = person.last_name();
    println!("{}", last_name);

    let title = person.title(None);
    println!("{}", title);
}
