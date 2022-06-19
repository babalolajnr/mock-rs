mod calculator;

pub mod provider;

pub use provider::en_us;

pub use provider::{
    address::AddressTrait,
    universal::barcode::BarcodeTrait,
    company::CompanyTrait,
    miscellaneous::MiscellaneousTrait,
    universal::payment::PaymentTrait,
    person::{Gender, PersonTrait},
    phone_number::PhoneNumberTrait,
};
