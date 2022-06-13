pub mod calculator;
pub mod provider;

mod tests {
    use crate::provider::{
        en_us::person::{Person},
        person::{Gender, Person as PersonTrait},
    };

    use super::*;

    #[test]
    fn test_can_get_random_name() {
        let en_us_person = Person::new();
        let name = en_us_person.name(Some(Gender::Male));
        assert!(name.len() > 0);
    }
}
