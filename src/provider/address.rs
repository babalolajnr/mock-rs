use super::base::Base;

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

    pub fn city_suffix(&self) -> &'a str {
        Self::random_element(&self.city_suffix)
    }

    pub fn street_suffix(&self) -> &'a str {
        Self::random_element(&self.street_suffix)
    }

    // pub fn building_number(&self) -> &'a str {
    //     Self::random_element(&self.building_number)
    // }
}

impl<'a> Base for Address<'a> {}
