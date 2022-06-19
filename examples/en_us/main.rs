use mock_rs::{en_us::Person, PersonTrait};

fn main() {
    let person = Person::new();

    let first_name = person.first_name(None);
    println!("{}", first_name);

    let last_name = person.last_name();
    println!("{}", last_name);

    let title = person.title(None);
    println!("{}", title);

    let first_name_male = person.first_name_male();
    println!("{}", first_name_male);
}
