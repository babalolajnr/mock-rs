mod calculator;

pub mod provider;
mod helpers;

pub use provider::en_us;

pub use provider::{
    address::AddressTrait,
    barcode::BarcodeTrait,
    company::CompanyTrait,
    miscellaneous::MiscellaneousTrait,
    payment::PaymentTrait,
    person::{Gender, PersonTrait},
    phone_number::PhoneNumberTrait,
};
