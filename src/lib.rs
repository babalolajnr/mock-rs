mod provider;

#[cfg(test)]
mod tests {
    use crate::provider::*;

    #[test]
    fn name_can_be_generated() {
        let name = person::name("male");
        println!("{}", name);
        let empty_string = "";
        assert_ne!(name, empty_string);
    }

    #[test]
    fn female_name_can_be_generated() {
        let name = person::name("female");
        let female_names = en_us::person::Person::new().first_name_female;
        println!("{}", name);
        assert!(female_names.contains(&name));
    }


}
