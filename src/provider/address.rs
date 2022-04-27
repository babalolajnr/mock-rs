use super::base::Base;

#[derive(Debug)]
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

impl<'a> Address<'a> {
    pub fn new() -> Self {
        Self {
            city_suffix: vec!["Ville"],
            street_suffix: vec!["Street"],
            city_formats: vec!["{{firstName}}{{citySuffix}}"],
            street_name_formats: vec!["{{lastName}} {{streetSuffix}}"],
            street_address_formats: vec!["{{buildingNumber}} {{streetName}}"],
            address_formats: vec!["{{streetAddress}} {{postcode}} {{city}}"],
            building_number: vec!["%#"],
            post_code: vec!["#####"],
            country: vec![],
        }
    }

    /// Get random city suffix
    fn city_suffix(&'a self) -> &'a str {
        Self::random_element(&self.city_suffix)
    }

    /// Get random street suffix
    fn street_suffix(&'a self) -> &'a str {
        Self::random_element(&self.street_suffix)
    }

    /// Get building number
    fn building_number(&'a self) -> String {
        Self::numerify(Some(Self::random_element(&self.building_number)))
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
