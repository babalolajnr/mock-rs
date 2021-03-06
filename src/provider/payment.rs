use crate::{
    calculator::{iban::checksum, luhn},
    helpers::{
        base::{numerify, random_digit, random_element, random_index, random_key, random_letter},
        miscellaneous::boolean,
    },
};
use chrono::{Datelike, TimeZone, Utc};
use rand::Rng;
use std::collections::HashMap;

pub trait PaymentTrait<'a> {
    fn card_vendors() -> Vec<&'a str> {
        vec![
            "Visa",
            "MasterCard",
            "American Express",
            "Discover Card",
            "Visa Retired",
            "JCB",
        ]
    }

    /// List of card brand masks for generating valid credit card numbers
    fn card_params() -> HashMap<&'a str, Vec<&'a str>> {
        let mut card_params = HashMap::new();

        card_params.insert(
            "Visa",
            vec![
                "4539###########",
                "4556###########",
                "4916###########",
                "4532###########",
                "4929###########",
                "40240071#######",
                "4485###########",
                "4716###########",
                "4##############",
            ],
        );

        card_params.insert(
            "Visa Retired",
            vec![
                "4539########",
                "4556########",
                "4916########",
                "4532########",
                "4929########",
                "40240071####",
                "4485########",
                "4716########",
                "4###########",
            ],
        );

        card_params.insert(
            "MasterCard",
            vec![
                "2221###########",
                "23#############",
                "24#############",
                "25#############",
                "26#############",
                "2720###########",
                "51#############",
                "52#############",
                "53#############",
                "54#############",
                "55#############",
            ],
        );

        card_params.insert("America Express", vec!["34############", "37############"]);

        card_params.insert("Discover Card", vec!["6011###########"]);

        card_params.insert("JCB", vec!["3528###########", "3589###########"]);

        card_params
    }

    fn iban_formats() -> HashMap<&'a str, Vec<(char, u8)>> {
        let mut iban_formats = HashMap::new();

        iban_formats.insert("AD", vec![('n', 4), ('n', 4), ('c', 12)]);
        iban_formats.insert("AE", vec![('n', 3), ('n', 16)]);
        iban_formats.insert("AL", vec![('n', 8), ('c', 16)]);
        iban_formats.insert("AT", vec![('n', 5), ('n', 11)]);
        iban_formats.insert("AZ", vec![('a', 4), ('c', 20)]);
        iban_formats.insert("BA", vec![('n', 3), ('n', 3), ('n', 8), ('n', 2)]);
        iban_formats.insert("BE", vec![('n', 3), ('n', 7), ('n', 2)]);
        iban_formats.insert("BG", vec![('a', 4), ('n', 4), ('n', 2), ('c', 8)]);
        iban_formats.insert("BH", vec![('a', 4), ('c', 14)]);
        iban_formats.insert(
            "BR",
            vec![('n', 8), ('n', 5), ('n', 10), ('a', 1), ('c', 1)],
        );
        iban_formats.insert("CH", vec![('n', 5), ('c', 12)]);
        iban_formats.insert("CR", vec![('n', 4), ('n', 14)]);
        iban_formats.insert("CY", vec![('n', 3), ('n', 5), ('c', 16)]);
        iban_formats.insert("CZ", vec![('n', 4), ('n', 6), ('n', 10)]);
        iban_formats.insert("DE", vec![('n', 8), ('n', 10)]);
        iban_formats.insert("DK", vec![('n', 4), ('n', 9), ('n', 1)]);
        iban_formats.insert("DO", vec![('c', 4), ('n', 20)]);
        iban_formats.insert("EE", vec![('n', 2), ('n', 2), ('n', 11), ('n', 1)]);
        iban_formats.insert("EG", vec![('n', 4), ('n', 4), ('n', 17)]);
        iban_formats.insert(
            "ES",
            vec![('n', 4), ('n', 4), ('n', 1), ('n', 1), ('n', 10)],
        );
        iban_formats.insert("FI", vec![('n', 6), ('n', 7), ('n', 1)]);
        iban_formats.insert("FR", vec![('n', 5), ('n', 5), ('c', 11), ('n', 2)]);
        iban_formats.insert("GB", vec![('a', 4), ('n', 6), ('n', 8)]);
        iban_formats.insert("GE", vec![('a', 2), ('n', 16)]);
        iban_formats.insert("GI", vec![('a', 4), ('c', 15)]);
        iban_formats.insert("GR", vec![('n', 3), ('n', 4), ('c', 16)]);
        iban_formats.insert("GT", vec![('c', 4), ('c', 20)]);
        iban_formats.insert("HR", vec![('n', 7), ('n', 10)]);
        iban_formats.insert(
            "HU",
            vec![('n', 3), ('n', 4), ('n', 1), ('n', 15), ('n', 1)],
        );
        iban_formats.insert("IE", vec![('a', 4), ('n', 6), ('n', 8)]);
        iban_formats.insert("IL", vec![('n', 3), ('n', 3), ('n', 13)]);
        iban_formats.insert("IS", vec![('n', 4), ('n', 2), ('n', 6), ('n', 10)]);
        iban_formats.insert("IT", vec![('a', 1), ('n', 5), ('n', 5), ('c', 12)]);
        iban_formats.insert("KW", vec![('a', 4), ('n', 22)]);
        iban_formats.insert("KZ", vec![('n', 3), ('c', 13)]);
        iban_formats.insert("LB", vec![('n', 4), ('c', 20)]);
        iban_formats.insert("LI", vec![('n', 5), ('c', 12)]);
        iban_formats.insert("LT", vec![('n', 5), ('n', 11)]);
        iban_formats.insert("LU", vec![('n', 3), ('c', 13)]);
        iban_formats.insert("LV", vec![('a', 4), ('c', 13)]);
        iban_formats.insert("MC", vec![('n', 5), ('n', 5), ('c', 11), ('n', 2)]);
        iban_formats.insert("MD", vec![('c', 2), ('c', 18)]);
        iban_formats.insert("ME", vec![('n', 3), ('n', 13), ('n', 2)]);
        iban_formats.insert("MK", vec![('n', 3), ('c', 10), ('n', 2)]);
        iban_formats.insert("MR", vec![('n', 5), ('n', 5), ('n', 11), ('n', 2)]);
        iban_formats.insert("MT", vec![('a', 4), ('n', 5), ('c', 18)]);
        iban_formats.insert(
            "MU",
            vec![('a', 4), ('n', 2), ('n', 2), ('n', 12), ('n', 3), ('a', 3)],
        );
        iban_formats.insert("NL", vec![('a', 4), ('n', 10)]);
        iban_formats.insert("NO", vec![('n', 4), ('n', 6), ('n', 1)]);
        iban_formats.insert("PK", vec![('a', 4), ('c', 16)]);
        iban_formats.insert("PL", vec![('n', 8), ('n', 16)]);
        iban_formats.insert("PS", vec![('a', 4), ('c', 21)]);
        iban_formats.insert("PT", vec![('n', 4), ('n', 4), ('n', 11), ('n', 2)]);
        iban_formats.insert("RO", vec![('a', 4), ('c', 16)]);
        iban_formats.insert("RS", vec![('n', 3), ('n', 13), ('n', 2)]);
        iban_formats.insert("SA", vec![('n', 2), ('c', 18)]);
        iban_formats.insert("SE", vec![('n', 3), ('n', 16), ('n', 1)]);
        iban_formats.insert("SI", vec![('n', 5), ('n', 8), ('n', 2)]);
        iban_formats.insert("SK", vec![('n', 4), ('n', 6), ('n', 10)]);
        iban_formats.insert("SM", vec![('a', 1), ('n', 5), ('n', 5), ('c', 12)]);
        iban_formats.insert("TN", vec![('n', 2), ('n', 3), ('n', 13), ('n', 2)]);
        iban_formats.insert("TR", vec![('n', 5), ('n', 1), ('c', 16)]);
        iban_formats.insert("VG", vec![('a', 4), ('n', 16)]);

        iban_formats
    }

    /// Returns a credit card vendor name
    fn credit_card_type() -> String {
        random_element(&Self::card_vendors()).to_string()
    }

    /// Returns the String of a credit card number.
    fn credit_card_number(
        credit_card_type: Option<&str>,
        formatted: Option<bool>,
        separator: Option<char>,
    ) -> String {
        let card_number;
        let default_card_param = vec!["2221###########"];

        let card_type = if let Some(card_type) = credit_card_type {
            if !Self::card_vendors().contains(&card_type) {
                panic!("{} is not a valid credit_card_type \n This is a list of the valid card types {:?}", 
                card_type, Self::card_vendors())
            }
            card_type
        } else {
            let card_vendors = Self::card_vendors();
            let random_index = random_index(&card_vendors);
            let card_type = card_vendors[random_index];
            card_type
        };

        let card_params = Self::card_params();

        let card_param = card_params.get(&card_type).unwrap_or(&default_card_param);

        let mask = random_element(&card_param);

        let mut number = numerify(Some(mask));
        number = format!("{}{}", number, luhn::compute_check_digit(&number));

        card_number = if let Some(true) = formatted {
            let separator = separator.unwrap_or('-');
            let p1 = number.chars().take(4).collect::<String>();
            let p2 = number.chars().skip(4).take(4).collect::<String>();
            let p3 = number.chars().skip(8).take(4).collect::<String>();
            let p4 = number.chars().skip(12).collect::<String>();
            format!(
                "{}{}{}{}{}{}{}",
                p1, separator, p2, separator, p3, separator, p4
            )
        } else {
            number
        };

        card_number
    }

    /// Returns a credit card expiration date.
    /// The expiration date is in the format MM/YY.
    fn credit_card_expiration_date(valid: Option<bool>) -> String {
        let valid = valid.unwrap_or(true);

        let random_day: u32 = rand::thread_rng().gen_range(1..28);
        let random_month: u32 = rand::thread_rng().gen_range(1..12);

        if valid {
            let now = Utc::now().year();
            let end = now + 3;
            let random_year: i32 = rand::thread_rng().gen_range(now..end);

            let date = Utc
                .ymd(random_year, random_month, random_day)
                .format("%m/%y");

            return format!("{}", date);
        }

        let now = Utc::now().year();
        let end = now - 3;
        let random_year: i32 = rand::thread_rng().gen_range(end..now);

        let date = Utc
            .ymd(random_year, random_month, random_day)
            .format("%m/%y");

        format!("{}", date)
    }

    /// Returns credit card details with
    ///
    /// * credit_card_type
    /// * credit_card_number
    /// * credit_card_expiration_date
    /// * holder_name
    ///
    /// Returns a HashMap
    fn credit_card_details(valid: Option<bool>) -> HashMap<&'a str, String> {
        let credit_card_type = Self::credit_card_type();
        let credit_card_number = Self::credit_card_number(Some(&credit_card_type), None, None);
        let name = "Hello";
        let expiration_date = Self::credit_card_expiration_date(valid);

        let mut details = HashMap::new();
        details.insert("credit_card_type", credit_card_type);
        details.insert("credit_card_number", credit_card_number);
        details.insert("holder_name", name.to_owned());
        details.insert("expiration_date", expiration_date);
        details
    }

    /// Get International Bank Account Number (IBAN)
    fn iban(country_code: Option<&str>, prefix: Option<&str>, mut length: Option<u8>) -> String {
        let country_code = country_code
            .unwrap_or(&random_key(&Self::iban_formats()).to_string())
            .to_uppercase();

        let iban_formats = Self::iban_formats();

        let mut format = if iban_formats.contains_key(country_code.as_str()) {
            iban_formats.get(country_code.as_str())
        } else {
            None
        };

        if let None = length {
            length = if let None = format {
                Some(24)
            } else {
                let mut length = 0;

                for part in format.unwrap() {
                    let (_class, group_count) = part;

                    length = length + group_count;
                }

                Some(length)
            };
        }

        let default_format = &vec![('n', length.unwrap())];

        if format == None {
            format = Some(default_format);
        }

        let mut expanded_format = "".to_string();

        for item in format.unwrap() {
            let (class, length) = item;
            let format_part = class.to_string().as_str().repeat(*length as usize);
            expanded_format.push_str(&format_part);
        }

        let mut result = prefix.unwrap_or("").to_string();
        expanded_format = (&expanded_format[result.len()..]).to_string();

        for class in expanded_format.chars() {
            match class {
                'c' => {
                    if boolean(None) {
                        result.push(random_digit() as char)
                    } else {
                        result.push_str(&random_letter().to_string().as_str().to_uppercase())
                    };
                }
                'a' => result.push_str(&random_letter().to_string().as_str().to_uppercase()),
                'n' => result.push(random_digit() as char),
                _ => result.push_str(&random_letter().to_string().as_str().to_uppercase()),
            }
        }

        let checksum = checksum(
            format!(
                "{country_code}00{result}",
                country_code = country_code,
                result = result
            )
            .as_str(),
        );

        format!("{}{}{}", country_code, checksum, result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    struct TestPay {}
    impl PaymentTrait<'_> for TestPay {}

    #[test]
    fn test_credit_card_type() {
        let card_type = TestPay::credit_card_type();
        assert!(TestPay::card_vendors().contains(&card_type.as_str()));
    }

    #[test]
    fn test_credit_card_number() {
        let card_number = TestPay::credit_card_number(None, None, None);
        assert_eq!(card_number.len(), 16);
    }

    #[test]
    fn test_credit_card_number_with_type() {
        let card_number = TestPay::credit_card_number(Some("MasterCard"), None, None);
        assert_eq!(card_number.len(), 16);
    }

    // TODO: Look into why some of these tests fail sometimes
    #[test]
    fn test_credit_card_number_with_separator() {
        let card_number = TestPay::credit_card_number(None, Some(true), Some('-'));
        println!("{}", card_number);
        assert_eq!(card_number.len(), 19);
    }

    #[test]
    fn test_card_expiration_date_with_valid_set_to_true() {
        let date = TestPay::credit_card_expiration_date(Some(true));
        assert_eq!(date.len(), 5);
    }

    #[test]
    fn test_card_expiration_date_with_valid_set_to_false() {
        let date = TestPay::credit_card_expiration_date(Some(false));
        assert_eq!(date.len(), 5);
    }

    #[test]
    fn test_card_details() {
        let details = TestPay::credit_card_details(Some(true));
        println!("{:?}", details);
        assert_eq!(details.len(), 4);
    }

    #[test]
    fn iban() {
        let iban = TestPay::iban(None, None, None);
        println!("{}", iban);
        assert!(iban.len() > 0);
    }
}
