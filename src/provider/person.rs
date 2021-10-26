use super::en_us::person;
use rand::prelude::*;

const GENDER_MALE: &str = "male";
const GENDER_FEMALE: &str = "female";

pub fn name(gender: Option<&str>) -> String {
    let person = person::Person::new();

    match gender {
        Some(GENDER_MALE) => {
            let first_name_random_index = generate_random_index(&person.first_name_male);
            let last_name_random_index = generate_random_index(&person.last_name);
            let mut male_name: String = String::from("");
            male_name.push_str(person.first_name_male[first_name_random_index]);
            male_name.push_str(" ");
            male_name.push_str(person.last_name[last_name_random_index]);
            male_name
        }

        Some(GENDER_FEMALE) => {
            let first_name_random_index = generate_random_index(&person.first_name_female);
            let last_name_random_index = generate_random_index(&person.last_name);
            let mut female_name = String::from("");
            female_name.push_str(person.first_name_female[first_name_random_index]);
            female_name.push_str(" ");
            female_name.push_str(person.last_name[last_name_random_index]);
            female_name
        }

        _ => {
            panic!("Unknown Gender")
        }
    }
}

pub fn first_name(gender: Option<&str>) -> String {
    let mut person = person::Person::new();

    match gender {
        Some(GENDER_FEMALE) => {
            let random_index = generate_random_index(&person.first_name_female);
            person.first_name_female[random_index].to_string()
        }
        Some(GENDER_MALE) => {
            let random_index = generate_random_index(&person.first_name_male);
            person.first_name_male[random_index].to_string()
        }
        None => {
            let mut male_female_first_names = person.first_name_male;
            male_female_first_names.append(&mut person.first_name_female);
            let random_index = generate_random_index(&male_female_first_names);
            male_female_first_names[random_index].to_string()
        }
        _ => {
            panic!("Unknown gender")
        }
    }
}

fn generate_random_index(arr: &[&str]) -> usize {
    let random_index = thread_rng().gen_range(1..arr.len());
    random_index
}
