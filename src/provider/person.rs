
use rand::prelude::*;

use super::en_us::person;

const GENDER_MALE: &str = "male";
const GENDER_FEMALE: &str = "female";

pub fn name(gender: Option<&str>) -> String {
    match gender {
        Some(GENDER_MALE) => {
            let male_name = first_name_male();
            let last_name = last_name();
            format!("{} {}", male_name, last_name)
        }

        Some(GENDER_FEMALE) => {
            let female_name = first_name_female();
            let last_name = last_name();
            format!("{} {}", female_name, last_name)
        }

        None => {
            let mut person = person::Person::new();
            let mut first_names = person.first_name_male;
            first_names.append(&mut person.first_name_female);
            format!(
                "{} {}",
                random_element(&first_names),
                random_element(&person.last_name)
            )
        }

        _ => {
            panic!("Unknown Gender")
        }
    }
}

pub fn first_name(gender: Option<&str>) -> String {
    match gender {
        Some(GENDER_FEMALE) => first_name_female(),
        Some(GENDER_MALE) => first_name_male(),
        None => {
            let mut person = person::Person::new();
            let mut names = person.first_name_male;
            names.append(&mut person.first_name_female);
            random_element(&names).to_string()
        }
        _ => {
            panic!("Unknown gender")
        }
    }
}

pub fn first_name_male() -> String {
    let person = person::Person::new();
    random_element(&person.first_name_male).to_string()
}

pub fn first_name_female() -> String {
    let person = person::Person::new();
    random_element(&person.first_name_female).to_string()
}

pub fn last_name() -> String {
    let person = person::Person::new();
    random_element(&person.last_name).to_string()
}

pub fn title(gender: Option<&str>) -> String {
    match gender {
        Some(GENDER_MALE) => random_element(&male_titles()).to_string(),
        Some(GENDER_FEMALE) => random_element(&female_titles()).to_string(),
        None => {
            let mut titles = male_titles();
            titles.append(&mut female_titles());
            random_element(&titles).to_string()
        }
        _ => {
            panic!("Unknown Gender")
        }
    }
}

fn male_titles() -> Vec<&'static str> {
    vec!["Mr.", "Dr.", "Prof."]
}

fn female_titles() -> Vec<&'static str> {
    vec!["Mrs.", "Ms.", "Miss", "Dr.", "Prof."]
}

fn generate_random_index<T>(arr: &[T]) -> usize {
    let random_index = thread_rng().gen_range(1..arr.len());
    random_index
}

fn random_element<T>(arr: &[T]) -> &T {
    let random_index = generate_random_index(&arr);
    &arr[random_index]
}

#[cfg(test)]
mod tests {
    use crate::provider::*;

    #[test]
    fn male_first_name_can_be_generated() {
        let name = super::first_name(Some("male"));
        let male_names = en_us::person::Person::new().first_name_male;
        assert!(male_names.contains(&name.as_str()));
    }

    #[test]
    fn female_first_name_can_be_generated() {
        let name = super::first_name(Some("female"));
        let female_names = en_us::person::Person::new().first_name_female;
        assert!(female_names.contains(&name.as_str()));
    }

    #[test]
    fn male_or_female_first_name_can_be_generated() {
        let name = super::first_name(None);
        let mut person = en_us::person::Person::new();
        let mut male_female_first_names = person.first_name_female;
        male_female_first_names.append(&mut person.first_name_male);
        assert!(male_female_first_names.contains(&name.as_str()));
    }

    #[test]
    #[should_panic(expected = "Unknown gender")]
    fn unknown_gender_will_panic() {
        person::first_name(Some("lorem"));
    }

    #[test]
    fn female_name_can_be_generated() {
        let name = super::name(Some("female"));
        let female_first_names = en_us::person::Person::new().first_name_female;
        let name: Vec<&str> = name.rsplit(" ").collect();
        assert!(female_first_names.contains(&name[1]));
    }

    #[test]
    fn male_name_can_be_generated() {
        let name = super::name(Some("male"));
        let male_first_names = en_us::person::Person::new().first_name_male;
        let name: Vec<&str> = name.rsplit(" ").collect();
        assert!(male_first_names.contains(&name[1]));
    }

    #[test]
    fn name() {
        let name = super::name(None);
        assert_ne!("", name);
    }

    #[test]
    fn first_name_male() {
        let first_name_male = super::first_name_male();
        let male_names = en_us::person::Person::new().first_name_male;
        assert!(male_names.contains(&first_name_male.as_str()));
    }

    #[test]
    fn first_name_female() {
        let first_name_female = super::first_name_female();
        let female_names = en_us::person::Person::new().first_name_female;
        assert!(female_names.contains(&first_name_female.as_str()));
    }

    #[test]
    fn last_name() {
        let last_name = super::last_name();
        let last_names = en_us::person::Person::new().last_name;
        assert!(last_names.contains(&last_name.as_str()));
    }

    #[test]
    fn title() {
        let title = super::title(None);
        assert_ne!(title, "");
    }

    #[test]
    fn male_title() {
        let male_title = super::title(Some("male"));
        let male_titles = super::male_titles();
        assert!(male_titles.contains(&male_title.as_str()));
    }

    #[test]
    fn female_title() {
        let female_title = super::title(Some("female"));
        let female_titles = super::female_titles();
        assert!(female_titles.contains(&female_title.as_str()));
    }
}
