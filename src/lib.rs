mod provider;

#[cfg(test)]
mod tests {
    use crate::provider::{
        en_us::person::Person as en_us_person,
        person::{self, Attributes},
    };
    
    #[test]
    fn can_get_fake_first_name() {
        let first_name = person::Person::new().first_name(None);
        assert!(first_name.len() > 0);
    }

    #[test]
    fn can_get_fake_last_name() {
        let last_name = person::Person::new().last_name();
        assert!(last_name.len() > 0);
    }

    #[test]
    fn can_get_fake_name() {
        let name = person::Person::new().name(None);
        assert!(name.len() > 0);
    }

    #[test]
    fn can_get_fake_en_us_name() {
        let name = en_us_person::new().name(None);
        assert!(name.len() > 0);
    }

    #[test]
    fn can_get_fake_title() {
        let title = person::Person::new().title(None);
        assert!(title.len() > 0);
    }

    #[test]
    fn can_get_fake_en_us_title() {
        let title = en_us_person::new().title(None);
        assert!(title.len() > 0);
    }

    // #[test]
    // fn can_get_fake_address() {
    //     let address = address::Address::new().;
    //     assert!(address.len() > 0);
    // }
}
