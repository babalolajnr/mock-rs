use crate::provider::{address::Address, person::Person};

pub struct Factory<'a> {
    pub address: Address<'a>,
    pub person: Person,
}

impl Factory<'_> {
    pub fn new() -> Self {
        Factory {
            address: Address::new(),
            person: Person::new(),
        }
    }
}
