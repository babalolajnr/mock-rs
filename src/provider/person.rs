pub trait PersonTrait {
    fn name(&self, gender: Option<Gender>) -> String;

    fn first_name(&self, gender: Option<Gender>) -> String;

    fn last_name(&self) -> String;

    fn first_name_male(&self) -> String;

    fn first_name_female(&self) -> String;

    fn title(&self, gender: Option<Gender>) -> String;

    fn title_male(&self) -> String;

    fn title_female(&self) -> String;
}

#[derive(Debug)]
pub enum Gender {
    Male,
    Female,
}
