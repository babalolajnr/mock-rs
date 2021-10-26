mod provider;

#[cfg(test)]
mod tests {
    use crate::provider::*;

    #[test]
    fn male_first_name_can_be_generated() {
        let name = person::first_name(Some("male"));
        let male_names = en_us::person::Person::new().first_name_male;
        println!("{}", name);
        assert!(male_names.contains(&name.as_str()));
    }

    #[test]
    fn female_first_name_can_be_generated() {
        let name = person::first_name(Some("female"));
        let female_names = en_us::person::Person::new().first_name_female;
        println!("{}", name);
        assert!(female_names.contains(&name.as_str()));
    }

    #[test]
    fn male_or_female_first_name_can_be_generated() {
        let name = person::first_name(None);
        let mut person = en_us::person::Person::new();
        let mut male_female_first_names = person.first_name_female;
        male_female_first_names.append(&mut person.first_name_male);
        println!("{}", name);
        assert!(male_female_first_names.contains(&name.as_str()));
    }

    #[test]
    #[should_panic(expected = "Unknown gender")]
    fn unknown_gender_will_panic() {
        person::first_name(Some("lorem"));
    }

    #[test]
    fn female_name_can_be_generated() {
        let name = person::name(Some("female"));
        let female_first_names = en_us::person::Person::new().first_name_female;
        println!("{}", name);
        let name: Vec<&str> = name.rsplit(" ").collect();
        assert!(female_first_names.contains(&name[1]));
    }
}
