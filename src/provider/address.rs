use super::base::Base;

#[derive(Debug)]
#[allow(dead_code)]
struct Address<'a> {
    city_suffix: Vec<&'a str>,
    street_suffix: Vec<&'a str>,
    city_formats: Vec<&'a str>,
    street_name_formats: Vec<&'a str>,
    street_address_formats: Vec<&'a str>,
    address_formats: Vec<&'a str>,
    building_number: Vec<&'a str>,
    post_code: Vec<&'a str>,
    country: Vec<&'a str>,
}

#[allow(dead_code)]
impl<'a> Address<'a> {
    pub fn new() -> Self {
        Self {
            city_suffix: vec!["Ville"],
            street_suffix: vec!["Street"],
            city_formats: vec!["{{first_name}}{{city_suffix}}"],
            street_name_formats: vec!["{{last_name}} {{street_suffix}}"],
            street_address_formats: vec!["{{building_number}} {{street_name}}"],
            address_formats: vec!["{{street_address}} {{postcode}} {{city}}"],
            building_number: vec!["%#"],
            post_code: vec!["#####"],
            country: vec![],
        }
    }

    /// Get random city suffix
    fn city_suffix(&'a self) -> &'a str {
        self.random_element(&self.city_suffix)
    }

    /// Get random street suffix
    fn street_suffix(&'a self) -> &'a str {
        self.random_element(&self.street_suffix)
    }

    /// Get building number
    fn building_number(&'a self) -> String {
        self.numerify(Some(self.random_element(&self.building_number)))
    }

    fn city() -> String {
        todo!()
    }
}

impl<'a> Base for Address<'a> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn city_suffix_works() {
        let address = Address::new();
        let result = address.city_suffix();
        assert!(result.len() > 0);
    }

    #[test]
    fn street_suffix_works() {
        let address = Address::new();
        let result = address.street_suffix();
        assert!(result.len() > 0);
    }

    #[test]
    fn building_number_works() {
        let address = Address::new();
        let result = address.building_number();
        assert!(result.len() > 0);
    }
}
