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

    /// Get building number
    fn building_number(&self) -> String {
        self.numerify(Some(self.random_element(&vec!["%#"])))
            .to_string()
    }

    fn post_code_format(&self) -> String {
        self.numerify(Some(self.random_element(&vec!["#####"])))
    }
}

trait Attributes: Base + Formats {
    /// Example: "town"
    fn city_suffix(&self) -> String {
        self.random_element(&vec!["Ville"]).to_string()
    }

    /// Example: "Avenue"
    fn street_suffix(&self) -> String {
        self.random_element(&vec!["Street"]).to_string()
    }

    /// Example: "Sashabury"
    fn city(&self) -> String {
        let format = &self.city_format();
        self.parse(format.as_str())
    }

    /// Example: "Crist Parks"
    fn street_name(&self) -> String {
        let format = &self.street_name_format();
        self.parse(format.as_str())
    }

    /// Example: "791 Crist Parks"
    fn street_address(&self) -> String {
        let format = &self.address_format();
        self.parse(format.as_str())
    }

    /// Example: "86039-9874"
    fn post_code(&self) -> String {
        self.bothify(Some(&self.post_code_format())).to_uppercase()
    }
}

impl Attributes for Address {}

impl Base for Address {
    fn call_method(&self, string: &str) -> Result<String, String> {
        match string {
            "city_suffix" => Ok(self.city_suffix()),
            "street_suffix" => Ok(self.street_suffix()),
            "building_number" => Ok(self.building_number()),
            "post_code" => Ok(self.post_code()),
            "street_name" => Ok(self.street_name()),
            "first_name" => Ok(self.first_name(None)),
            "last_name" => Ok(self.last_name()),
            "street_address" => Ok(self.street_address()),
            _ => Err(format!("Method '{}' not found", string)),
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

    #[test]
    fn street_name() {
        let address = Address::new();
        let result = address.street_name();
        assert!(result.len() > 0);
    }

    #[test]
    fn street_address() {
        let address = Address::new();
        let result = address.street_address();
        assert!(result.len() > 0);
    }

    #[test]
    fn post_code() {
        let address = Address::new();
        let result = address.post_code();
        println!("{}", result);
        assert!(result.len() > 0);
    }
}
