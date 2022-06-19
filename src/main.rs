use mock_rs::provider::{
    en_us::person::Person,
    person::{Gender, PersonTrait},
};

fn main() {
    let first_name = Person::new().first_name(Some(Gender::Male));

    println!("{}", first_name);
}
