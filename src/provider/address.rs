use crate::error::Errors;

use super::{
    base::Base,
    person::{Attributes as PersonAttributes, Formats as PersonFormats},
};

#[derive(Debug)]
pub struct Address {}

impl Address {
    pub fn new() -> Self {
        Self {}
    }
}

impl Formats for Address {}

trait Formats: Base {
    fn street_name_format(&self) -> String {
        self.random_element(&vec!["{{last_name}} {{street_suffix}}"])
            .to_string()
    }

    fn address_format(&self) -> String {
        self.random_element(&vec!["{{street_address}} {{postcode}} {{city}}"])
            .to_string()
    }

    fn city_format(&self) -> String {
        self.random_element(&vec!["{{first_name}} {{city_suffix}}"])
            .to_string()
    }
}

trait Attributes: Base + Formats {
    /// Get random city suffix
    fn city_suffix(&self) -> String {
        self.random_element(&vec!["Ville"]).to_string()
    }

    /// Get random street suffix
    fn street_suffix(&self) -> String {
        self.random_element(&vec!["Street"]).to_string()
    }

    /// Get building number
    fn building_number(&self) -> String {
        self.numerify(Some(self.random_element(&vec!["%#"])))
            .to_string()
    }

    fn post_code(&self) -> String {
        self.numerify(Some(self.random_element(&vec!["%####"])))
    }
    fn city(&self) -> String {
        let format = &self.city_format();
        self.parse(format.as_str())
    }

    fn street_name(&self) -> String {
        let format = &self.street_name_format();
        self.parse(format.as_str())
    }
}

impl Attributes for Address {}

impl Base for Address {
    fn call_method(&self, string: &str) -> Result<String, crate::error::Errors> {
        match string {
            "city_suffix" => Ok(self.city_suffix()),
            "street_suffix" => Ok(self.street_suffix()),
            "building_number" => Ok(self.building_number()),
            "post_code" => Ok(self.post_code()),
            "first_name" => Ok(self.first_name(None)),
            _ => Err(Errors::MethodNotFoundError),
        }
    }
}

impl PersonAttributes for Address {}
impl PersonFormats for Address {}

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

    #[test]
    fn city() {
        let address = Address::new();
        let result = address.city();
        assert!(result.len() > 0);
    }
}
