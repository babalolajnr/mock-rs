use crate::error::Errors;

use super::base::Base;

pub enum Gender {
    Male,
    Female,
}
pub struct Person {}

impl<'a> Person {
    fn first_name(&self) -> String {
        "John".to_string()
    }

    pub fn new() -> Self{
        Self{}
    }

    fn title_format(&self) -> String {
        self.random_element(&vec!["{{title_male}}", "{{title_female}}"]).to_string()
    }

    fn first_name_format(&self) -> String {
        self.random_element(&vec!["{{first_name_male}}", "{{first_name_female}}"]).to_string()
    }

    fn male_name_formats(&self) -> String {
        self.random_element(&vec!["{{first_name_male}} {{last_name}}"]).to_string()

    }

    fn female_name_formats(&self) -> String {
        self.random_element(&vec!["{{first_name_female}} {{last_name}}"]).to_string()
    }

    fn first_name_male(&self) -> String {
        "John".to_string()
    }

    fn first_name_female(&self) -> String {
        "Jane".to_string()
    }

    fn last_name(&self) -> String {
        "Doe".to_string()
    }

    fn title_male(&self) -> String {
        self.random_element(&vec!["Mr.", "Dr.", "Prof."]).to_string()
    }

    fn title_female(&self) -> String {
        self.random_element(&vec!["Mrs.", "Ms.", "Miss", "Dr.", "Prof."]).to_string()

    }

    /// Get a random name
    pub fn name(&self, gender: Option<Gender>) -> String {
        let format = match gender {
            Some(Gender::Male) => self.male_name_formats(),
            Some(Gender::Female) => self.female_name_formats(),
            None => {
                let male_name_format = &self.male_name_formats();
                let female_name_format = &self.female_name_formats();
                let merged = vec![male_name_format, female_name_format];
                self.random_element(&merged).to_string()
            }
        };

        self.parse(&format)
    }
}

impl Base for Person {
    fn call_method(&self, string: &str) -> Result<String, Errors> {
        println!("{}", string);
        match string {
            "first_name_male" => Ok(self.first_name_male().to_string()),
            "first_name_female" => Ok(self.first_name_female().to_string()),
            "last_name" => Ok(self.last_name().to_string()),
            "title_male" => Ok(self.title_male().to_string()),
            "title_female" => Ok(self.title_female().to_string()),
            _ => Err(Errors::MethodNotFoundError),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn name() {
        let person = Person::new().name(None);
        assert!(person.len() > 0);

        println!("{}", person);
    }
}
