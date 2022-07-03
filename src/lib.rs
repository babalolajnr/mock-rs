mod calculator;

mod helpers;
pub mod provider;

pub use provider::universal::{barcode::Barcode, payment::Payment};
pub use provider::en_us;

pub use provider::{
    address::AddressTrait,
    barcode::BarcodeTrait,
    company::CompanyTrait,
    payment::PaymentTrait,
    person::{Gender, PersonTrait},
    phone_number::PhoneNumberTrait,
};
