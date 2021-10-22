mod provider;

#[cfg(test)]
mod tests {
    use crate::provider::*;

    #[test]
    fn first_name_can_be_generated() {
        let name = person::first_name(Some("male"));
        println!("{}", name);
        let empty_string = "";
        assert_ne!(name, empty_string);
    }

    #[test]
    fn female_first_name_can_be_generated() {
        let name = person::first_name(Some("female"));
        let female_names = en_us::person::Person::new().first_name_female;
        println!("{}", name);
        assert!(female_names.contains(&name));
    }

    #[test]
    fn male_or_female_first_name_can_be_generated() {
        let name = person::first_name(None);
        let mut person = en_us::person::Person::new();
        let mut male_female_first_names = person.first_name_female;
        male_female_first_names.append(&mut person.first_name_male);
        println!("{}", name);
        assert!(male_female_first_names.contains(&name));
    }
}
