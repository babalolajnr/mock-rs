pub mod base;
pub mod en_us;
pub mod person;

pub enum Locale<'a> {
    EnUS(en_us::provider::Provider<'a>),
}