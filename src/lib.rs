mod provider;

#[cfg(test)]
mod tests {
    use crate::provider::person;

    #[test]
    fn name_can_be_generated() {
        let name = person::name("male");
        println!("{}", name);
        let empty_string = "";
        assert_ne!(name, empty_string);
    }
}
