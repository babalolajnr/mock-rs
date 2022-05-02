use super::{person::Gender, base::Base};

pub trait SharedFormats: Base {
    fn male_name_format(&self) -> String {
        self.random_element(&vec!["{{first_name_male}} {{last_name}}"])
            .to_string()
    }

    fn female_name_format(&self) -> String {
        self.random_element(&vec!["{{first_name_female}} {{last_name}}"])
            .to_string()
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
                let male_name_format = &self.male_name_format();
                let female_name_format = &self.female_name_format();
                let merged = vec![male_name_format, female_name_format];
                self.parse(&self.random_element(&merged).to_string())
            }
        }
    }
}