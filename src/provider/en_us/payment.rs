use rand::Rng;

use crate::provider::{base::BaseTrait, universal::payment};

pub struct Payment {}

impl Payment {
    /// Returns a random bank account number
    pub fn bank_account_number() -> String {
        // Length between 5 and 17
        let length: usize = rand::thread_rng().gen_range(5..17);
        let mask = "#".repeat(length);

        Self::numerify(Some(&mask))
    }

    /// Returns a random bank routing number
    pub fn bank_routing_number() -> String {
        // Length between 9 and 11
        let district: usize = rand::thread_rng().gen_range(1..12);
        let vec = &vec![&0, &0, &0, &0, &20, &20, &60];
        let district_type = Self::random_element(vec);
        let state: usize = rand::thread_rng().gen_range(1..9);
        let clearing_center: usize = rand::thread_rng().gen_range(1..9);
        let institution: usize = rand::thread_rng().gen_range(1000..9999);

        let result = format!(
            "{:2}{:1}{:1}{:4}",
            district + district_type,
            state,
            clearing_center,
            institution
        );

        format!("{}{}", result, Self::calculate_routing_number_checksum(&result))
    }

    fn calculate_routing_number_checksum(routing: &str) -> usize {
        let routing = routing.chars().collect::<Vec<char>>();
        let checksum: usize = (7
            * (routing[0] as usize + routing[3] as usize + routing[6] as usize)
            + 3 * (routing[1] as usize + routing[4] as usize + routing[7] as usize)
            + 9 * (routing[2] as usize + routing[5] as usize))
            % 10;

        checksum
    }
}
impl payment::PaymentTrait<'_> for Payment {}
impl BaseTrait for Payment {}

mod tests {

    #[test]
    fn test_bank_account_number() {
        let number = super::Payment::bank_account_number();
        assert!(number.len() >= 5);
        assert!(number.len() <= 17);
    }

    #[test]
    fn test_calculate_routing_number_checksum() {
        let routing = "10892900";
        let checksum = super::Payment::calculate_routing_number_checksum(&routing);
        println!("{}", checksum);
        // assert!(checksum <= 9);
    }

    #[test]
    fn test_bank_routing_number() {
        let number = super::Payment::bank_routing_number();
        println!("{}", number);
        assert!(number.len() >= 9);
        assert!(number.len() <= 11);
    }
}
