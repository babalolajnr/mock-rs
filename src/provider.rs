pub mod base;
pub mod en_us;
pub mod person;
mod address;

pub enum Locale {
    EnUS,
}

pub trait Provider {}
