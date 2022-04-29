mod factory;
mod provider;

#[cfg(test)]
mod tests{
    use crate::factory;

    #[test]
    fn can_get_fake_name() {
        let mocker = factory::Factory::new();
        
        let name = mocker.person.first_name();

        assert!(name.len() > 0);
        println!("{}", name);
    }
}