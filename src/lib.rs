mod calculator;

mod helpers;
pub mod provider;

pub use provider::en_us;
pub use provider::universal::{barcode::Barcode, color::Color, payment::Payment};

pub use provider::{
    address::AddressTrait,
    barcode::BarcodeTrait,
    color::ColorTrait,
    company::CompanyTrait,
    payment::PaymentTrait,
    person::{Gender, PersonTrait},
    phone_number::PhoneNumberTrait,
};
