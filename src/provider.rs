pub mod address;
pub mod base;
pub mod en_us;
pub mod formatter;
pub mod locale;
pub mod person;

pub trait Provider {
    fn name<'a>(&self) -> &'a str;
}
