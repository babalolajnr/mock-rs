mod calculator;

pub mod provider;
pub use provider::en_us::{
    address::Address, company::Company, payment::Payment, person::Person, phone_number::PhoneNumber,
};
pub use provider::{
    address::AddressTrait,
    barcode::BarcodeTrait,
    company::CompanyTrait,
    miscellaneous::MiscellaneousTrait,
    payment::PaymentTrait,
    person::{Gender, PersonTrait},
    phone_number::PhoneNumberTrait,
};

// mod tests {
// use crate::provider::{
//     en_us::person::Person,
//     person::{Gender, PersonTrait},
// };

// use super::*;

// #[test]
// fn test_can_get_random_name() {
//     let en_us_person = Person::new();
//     let name = en_us_person.name(Some(Gender::Male));
//     assert!(name.len() > 0);
// }
// }
