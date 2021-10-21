use super::en_us::person;

pub struct Person {}

const GENDER_MALE: &str = "male";
const GENDER_FEMALE: &str = "female";

impl Person {
    pub fn name(gender: &str) -> &str {
        if gender == GENDER_FEMALE {
            let person = person::Person::new();
            person.first_name_male[0]
        } else if gender == GENDER_MALE {
            let person = person::Person::new();
            person.first_name_male[0]
        }else{
            panic!("Gender must be male or female")
        }
    }
}