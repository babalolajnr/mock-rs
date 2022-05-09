use super::base::Base;

pub enum Gender {
    Male,
    Female,
}
pub struct Person {} 

impl<'a> Person {
    pub fn new() -> Self {
        Self {}
    }
}

impl Base for Person {
    fn call_method(&self, string: &str) -> Result<String, String> {
        match string {
            "first_name_male" => Ok(self.first_name_male().to_string()),
            "first_name_female" => Ok(self.first_name_female().to_string()),
            "last_name" => Ok(self.last_name().to_string()),
            "title_male" => Ok(self.title_male().to_string()),
            "title_female" => Ok(self.title_female().to_string()),
            _ => Err(format!("Method '{}' not found", string)),
        }
    }
}

impl Attributes for Person {}

pub trait Attributes: Base + Formats {
    fn last_name(&self) -> String {
        "Doe".to_string()
    }

    fn title_female(&self) -> String {
        self.random_element(&vec!["Mrs.", "Ms.", "Miss", "Dr.", "Prof."])
            .to_string()
    }

    fn title_male(&self) -> String {
        self.random_element(&vec!["Mr.", "Dr.", "Prof."])
            .to_string()
    }

    /// Get a random name
    fn name(&self, gender: Option<Gender>) -> String {
        let format = match gender {
            Some(Gender::Male) => self.male_name_format(),
            Some(Gender::Female) => self.female_name_format(),
            None => {
                let male_name_format = &self.male_name_format();
                let female_name_format = &self.female_name_format();
                let merged = vec![male_name_format, female_name_format];
                self.random_element(&merged).to_string()
            }
        };

        self.parse(&format)
    }

    fn title(&self, gender: Option<Gender>) -> String {
        match gender {
            Some(Gender::Male) => self.title_male(),
            Some(Gender::Female) => self.title_female(),
            None => {
                let title = &self.title_format();
                self.parse(&title)
            }
        }
    }

    fn first_name_male(&self) -> String {
        "John".to_string()
    }

    fn first_name_female(&self) -> String {
        "Jane".to_string()
    }

    /// Get random first name
    fn first_name(&self, gender: Option<Gender>) -> String {
        match gender {
            Some(Gender::Male) => self.first_name_male(),
            Some(Gender::Female) => self.first_name_female(),
            None => {
                let male_name_format = &self.first_name_male();
                let female_name_format = &self.first_name_female();
                let merged = vec![male_name_format, female_name_format];
                self.random_element(&merged).to_string()
            }
        }
    }
}

impl Formats for Person {}

pub trait Formats: Base {
    fn first_name_format(&self) -> String {
        self.random_element(&vec!["{{first_name_male}}", "{{first_name_female}}"])
            .to_string()
    }

    fn title_format(&self) -> String {
        self.random_element(&vec!["{{title_male}}", "{{title_female}}"])
            .to_string()
    }

    fn male_name_format(&self) -> String {
        self.random_element(&vec!["{{first_name_male}} {{last_name}}"])
            .to_string()
    }

    fn female_name_format(&self) -> String {
        self.random_element(&vec!["{{first_name_female}} {{last_name}}"])
            .to_string()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn name() {
        let person = Person::new().name(None);
        assert!(person.len() > 0);
    }

    #[test]
    fn name_with_gender_some_option_works() {
        let person = Person::new().name(Some(Gender::Female));
        assert!(person.len() > 0);
    }

    #[test]
    fn first_name() {
        let person = Person::new().first_name(None);
        assert!(person.len() > 0);
    }

    #[test]
    fn title() {
        let person = Person::new().title(None);
        assert!(person.len() > 0);
    }
}
