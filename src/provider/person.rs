use super::en_us::person;
use rand::prelude::*;

const GENDER_MALE: &str = "male";
const GENDER_FEMALE: &str = "female";

pub fn name(gender: &str) -> &str {
    if gender == GENDER_FEMALE {
        let person = person::Person::new();
        let random_number = generate_random_number(&person.first_name_male);
        person.first_name_male[random_number]
    } else if gender == GENDER_MALE {
        let person = person::Person::new();
        let random_number = generate_random_number(&person.first_name_male);
        person.first_name_male[random_number]
    } else {
        panic!("Gender must be male or female")
    }
}

fn generate_random_number(arr: &[&str]) -> usize {
    let random_number = thread_rng().gen_range(1..arr.len());
    random_number
}
