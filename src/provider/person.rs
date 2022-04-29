use super::base::Base;

enum Gender {
    Male,
    Female,
}
pub struct Person {
}

impl<'a> Person {
    fn first_name() -> &'a str {
        "John"
    }

    fn title_format() -> Vec<&'a str> {
        vec!["{{title_male}}", "{{title_female}}"]
    }

    fn first_name_format() -> Vec<&'a str> {
        vec!["{{first_name_male}}", "{{first_name_female}}"]
    }

    fn male_name_formats() -> Vec<&'a str> {
        vec!["{{first_name_male}} {{last_name}}"]
    }

    fn female_name_formats() -> Vec<&'a str> {
        vec!["{{first_name_female}} {{last_name}}"]
    }

    fn first_name_male() -> &'static str {
        "John"
    }

    fn first_name_female() -> &'static str {
        "Jane"
    }

    fn last_name() -> &'static str {
        "Doe"
    }

    fn title_male() -> Vec<&'a str> {
        vec!["Mr.", "Dr.", "Prof."]
    }

    fn title_female() -> Vec<&'a str> {
        vec!["Mrs.", "Ms.", "Miss", "Dr.", "Prof."]
    }

    /// Get a random name 
    pub fn name(&self, gender: Option<Gender>) -> &'a str{

       let format =  match gender {
            Some(Gender::Male) =>  Self::random_element(&self, &Self::male_name_formats()),
            Some(Gender::Female) =>  Self::random_element(&self, &Self::female_name_formats()),
            None => {
                let merged = vec![];
                merged.append(&mut Self::male_name_formats());
                merged.append(&mut Self::female_name_formats());

                Self::random_element(&self, &merged)
            },
        };

        &self.parse(format)
    }
}

impl Base for Person{}

#[cfg(test)]
mod tests {

    use super::*;

    // #[test]
    // fn first_name() {
    //     let person = Person::first_name()
    // }
}
