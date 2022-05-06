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

trait Attributes: Base {
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
        format!("{} {}", "John", self.city_suffix())
    }

    /// Example: "Crist Parks"
    fn street_name(&self) -> String {
        format!("{} {}", "Babalola", self.street_suffix())
    }

    /// Example: "791 Crist Parks"
    fn street_address(&self) -> String {
        format!("{} {}", self.numerify(Some("##")), self.street_name())
    }

    /// Example: "86039-9874"
    fn postcode(&self) -> String {
        self.numerify(Some("#####"))
    }

    /// Example: "791 Crist Parks, Sashabury, IL 86039-9874"
    fn address(&self) -> String {
        format!(
            "{} {} {}",
            self.street_address(),
            self.postcode(),
            self.city()
        )
    }

    fn building_number(&self) -> String {
        self.numerify(Some("##"))
    }
}

impl Attributes for Address {}

impl Base for Address {
    fn call_method(&self, string: &str) -> Result<String, String> {
        match string {
            "city_suffix" => Ok(self.city_suffix()),
            "street_suffix" => Ok(self.street_suffix()),
            "building_number" => Ok(self.building_number()),
            "postcode" => Ok(self.postcode()),
            "street_name" => Ok(self.street_name()),
            "first_name" => Ok(self.first_name(None)),
            "last_name" => Ok(self.last_name()),
            "street_address" => Ok(self.street_address()),
            "city" => Ok(self.city()),
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
    fn postcode() {
        let address = Address::new();
        let result = address.postcode();
        assert!(result.len() > 0);
    }

    #[test]
    fn address() {
        let address = Address::new();
        let result = address.address();
        println!("{}", result);
        assert!(result.len() > 0);
    }
}
