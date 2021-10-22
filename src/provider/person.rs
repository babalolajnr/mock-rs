use super::en_us::person;
use rand::prelude::*;

const GENDER_MALE: &str = "male";
const GENDER_FEMALE: &str = "female";

// pub fn name(gender: &str) -> &str {

// }

pub fn first_name(gender: Option<&str>) -> &str {
    let mut person = person::Person::new();

    match gender {
        Some(GENDER_FEMALE) => {
            let random_number = generate_random_number(&person.first_name_female);
            person.first_name_female[random_number]
        }
        Some(GENDER_MALE) => {
            let random_number = generate_random_number(&person.first_name_male);
            person.first_name_male[random_number]
        }
        None => {
            let mut male_female_first_names = person.first_name_male;
            male_female_first_names.append(&mut person.first_name_female);
            let random_number = generate_random_number(&male_female_first_names);
            male_female_first_names[random_number]
        }
        _ => {
            panic!("Unknown gender")
        }
    }
}

fn generate_random_number(arr: &[&str]) -> usize {
    let random_number = thread_rng().gen_range(1..arr.len());
    random_number
}
