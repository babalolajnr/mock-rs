use std::collections::HashMap;

use crate::calculator::luhn;

use super::base::Base;

pub trait Payment<'a>: Base {
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
        Self::random_element(&Self::card_vendors()).to_string()
    }

    /// Returns the String of a credit card number.
    fn credit_card_number(
        credit_card_type: Option<&str>,
        formatted: Option<bool>,
        separator: Option<char>,
    ) -> String {
        let card_number;

        let card_type = match credit_card_type {
            Some(card_type) => {
                if !Self::card_vendors().contains(&card_type) {
                    panic!("{} is not a valid credit_card_type \n This is a list of the valid card types {:?}", card_type, Self::card_vendors())
                }
                card_type
            }
            None => {
                let card_vendors = Self::card_vendors();
                let random_index = Self::random_index(&card_vendors);
                let card_type = card_vendors[random_index];
                card_type
            }
        };

        let card_params = Self::card_params();

        let card_param = match card_params.get(&card_type) {
            Some(card_param) => card_param,
            None => card_params.get("MasterCard").unwrap(),
        };

        let mask = Self::random_element(&card_param);

        let mut number = Self::numerify(Some(mask));
        number = format!("{}{}", number, luhn::compute_check_digit(&number));

        card_number = match formatted {
            Some(true) => {
                let separator = separator.unwrap_or('-');
                let p1 = number.chars().take(4).collect::<String>();
                let p2 = number.chars().skip(4).take(4).collect::<String>();
                let p3 = number.chars().skip(8).take(4).collect::<String>();
                let p4 = number.chars().skip(12).collect::<String>();
                format!(
                    "{}{}{}{}{}{}{}",
                    p1, separator, p2, separator, p3, separator, p4
                )
            }
            _ => number,
        };

        card_number
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    struct TestPay {}
    impl Payment<'_> for TestPay {}
    impl Base for TestPay {}

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
}
