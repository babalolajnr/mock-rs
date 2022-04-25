use super::person::Person;

pub struct Provider<'a> {
    pub person: Person<'a>,
}

impl<'a> Provider<'a> {
    pub fn new() -> Self {
        Self {
            person: Person::new(),
        }
    }
}
