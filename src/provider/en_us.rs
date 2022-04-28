use super::locale::Locale;

pub mod person;
pub mod provider;
pub mod address;

pub(crate) struct EnUS {}

impl Locale for EnUS {
    fn name<'a>(&self) -> &'a str {
        "en_us"
    }
}

impl EnUS {
    pub fn new() -> Self {
        Self {}
    }
}
