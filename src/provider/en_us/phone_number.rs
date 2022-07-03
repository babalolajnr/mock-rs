use rand::{thread_rng, Rng};

use crate::helpers::base::{random_element, numerify};


pub struct PhoneNumber {
    formats: Vec<&'static str>,
    formats_with_extension: Vec<&'static str>,
    e_164_format: &'static str,
    toll_free_area_codes: Vec<&'static str>,
    toll_free_formats: Vec<&'static str>,
}

impl PhoneNumber {
    pub fn new() -> Self {
        Self {
            formats: vec![
                // International format
                "+1-{area_code}-{exchange_code}-{prefix}",
                "+1 ({area_code}) {exchange_code}-{prefix}",
                "+1-{area_code}-{exchange_code}-{prefix}",
                "+1.{area_code}.{exchange_code}.{prefix}",
                "+1{area_code}{exchange_code}{prefix}",
                // Standard formats
                "{area_code}-{exchange_code}-{prefix}",
                "({area_code}) {exchange_code}-{prefix}",
                "1-{area_code}-{exchange_code}-{prefix}",
                "{area_code}.{exchange_code}.{prefix}",
                "{area_code}-{exchange_code}-{prefix}",
                "({area_code}) {exchange_code}-{prefix}",
                "1-{area_code}-{exchange_code}-{prefix}",
                "{area_code}.{exchange_code}.{prefix}",
            ],

            formats_with_extension: vec![
                "{area_code}-{exchange_code}-{prefix} x###",
                "({area_code}) {exchange_code}-{prefix} x###",
                "1-{area_code}-{exchange_code}-{prefix} x###",
                "{area_code}.{exchange_code}.{prefix} x###",
                "{area_code}-{exchange_code}-{prefix} x####",
                "({area_code}) {exchange_code}-{prefix} x####",
                "1-{area_code}-{exchange_code}-{prefix} x####",
                "{area_code}.{exchange_code}.{prefix} x####",
                "{area_code}-{exchange_code}-{prefix} x#####",
                "({area_code}) {exchange_code}-{prefix} x#####",
                "1-{area_code}-{exchange_code}-{prefix} x#####",
                "{area_code}.{exchange_code}.{prefix} x#####",
            ],

            e_164_format: "+1{area_code}{exchange_code}####",

            toll_free_area_codes: vec!["800", "844", "855", "866", "877", "888"],

            toll_free_formats: vec![
                // Standard formats
                "{toll_free_area_code}-{exchange_code}-####",
                "({toll_free_area_code}) {exchange_code}-####",
                "1-{toll_free_area_code}-{exchange_code}-####",
                "{toll_free_area_code}.{exchange_code}.####",
            ],
        }
    }

    /// Generate a random phone number.
    pub fn phone_number(&self) -> String {
        let format = random_element(&self.formats);
        self.parse_format(format)
    }

    /// Generate a random phone number with an extension.
    pub fn phone_number_with_extension(&self) -> String {
        let format = random_element(&self.formats_with_extension);
        self.parse_format(&format)
    }

    /// Generate a random toll-free phone number.
    pub fn toll_free_phone_number(&self) -> String {
        let format = random_element(&self.toll_free_formats);
        self.parse_format(&format)
    }

    /// Generate a random phone number in E.164 format.
    pub fn e_164_phone_number(&self) -> String {
        self.parse_format(&self.e_164_format)
    }

    /// Parse a format string
    fn parse_format(&self, format: &str) -> String {
        let mut format = format.to_string();
        format = format.replace("{area_code}", &self.area_code());
        format = format.replace("{exchange_code}", &self.exchange_code());
        format = format.replace("{prefix}", &numerify(Some("####")));
        format = format.replace(
            "{toll_free_area_code}",
            random_element(&self.toll_free_area_codes),
        );

        if format.contains("#") {
            return numerify(Some(&format));
        }

        format
    }

    /// Generate a random area code.
    fn area_code(&self) -> String {
        let exchange_code: u32 = thread_rng().gen_range(200..1000);
        exchange_code.to_string()
    }

    ///Returns random exchange_code
    fn exchange_code(&self) -> String {
        let exchange_code: u32 = thread_rng().gen_range(200..1000);
        exchange_code.to_string()
    }
}

mod tests {

    #[test]
    fn test_phone_number() {
        let phone_number = super::PhoneNumber::new().phone_number();
        println!("{}", phone_number);
        assert!(phone_number.len() > 1);
    }

    #[test]
    fn test_phone_number_with_extension() {
        let phone_number = super::PhoneNumber::new().phone_number_with_extension();
        println!("{}", phone_number);
        assert!(phone_number.len() > 1);
    }

    #[test]
    fn test_e_164_phone_number() {
        let phone_number = super::PhoneNumber::new().e_164_phone_number();
        println!("{}", phone_number);
        assert!(phone_number.len() > 1);
    }

    #[test]
    fn toll_free_phone_number() {
        let phone_number = super::PhoneNumber::new().toll_free_phone_number();
        println!("{}", phone_number);
        assert!(phone_number.len() > 1);
    }

    #[test]
    fn test_exchange_code() {
        let phone_number = super::PhoneNumber::new();
        let exchange_code = phone_number.exchange_code();
        assert!(exchange_code.len() == 3);
    }

    #[test]
    fn test_area_code() {
        let phone_number = super::PhoneNumber::new();
        let area_code = phone_number.area_code();
        assert!(area_code.len() == 3);
    }
}
