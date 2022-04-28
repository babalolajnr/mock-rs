use std::collections::HashMap;

use crate::{
    generator::Generator,
    provider::{
        en_us::EnUS,
        locale::Locale,
        person::Person,
        address::Address,
        Provider,
    },
};

struct Factory<'a> {
    providers: HashMap<&'a str, Box<dyn Provider>>,
    locale: Box<dyn Locale>,
}

impl<'a> Factory<'a> {
    pub fn new(locale: Option<Box<dyn Locale>>) -> Self {
        let locale = locale.unwrap_or(Box::new(EnUS::new()));

        let mut providers = HashMap::new();

        let person = Box::new(Person::new());
        let address = Box::new(Address::new());

        providers.insert(person.name(), person as Box<dyn Provider>);
        providers.insert(address.name(), address as Box<dyn Provider>);

        Self { locale, providers }
    }

    fn create(self) -> Generator<'a> {
        let mut generator = Generator::new();

        self.providers.into_iter().for_each(|(name, provider)| {
            generator.add_provider(name, provider);
        });

        generator
    }
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test_factory_new() {
        let factory = Factory::new(None);
        assert_eq!(factory.locale.name(), "en_us");
    }

    #[test]
    fn test_factory_create() {
        let factory = Factory::new(None);
        let generator = factory.create();
        assert!(generator.get_providers().contains_key("person"));
        assert!(generator.get_providers().contains_key("address"));
    }
}
