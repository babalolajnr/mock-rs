enum Gender {
    Male,
    Female,
}
pub struct Person {
    gender: Gender,
}


impl<'a> Person {
    pub fn first_name(&self) -> &'a str {
        "John"
    }
    
    pub fn new() -> Self {
        Self {
            gender: Gender::Male,
        }
    }

    pub fn title_format(&self) -> Vec<&'a str> {
        vec!["{{title_male}}", "{{title_female}}"]
    }

    pub fn first_name_format(&self) -> Vec<&'a str> {
        vec!["{{first_name_male}}", "{{first_name_female}}"]
    }

    pub fn male_name_formats(&self) -> Vec<&'a str> {
        vec!["{{first_name_male}} {{last_name}}"]
    }

    pub fn female_name_formats(&self) -> Vec<&'a str> {
        vec!["{{first_name_female}} {{last_name}}"]
    }

    pub fn first_name_male(&self) -> &'static str {
        "John"
    }

    pub fn first_name_female(&self) -> &'static str {
        "Jane"
    }

    pub fn last_name(&self) -> &'static str {
        "Doe"
    }

    pub fn title_male(&self) -> Vec<&'a str> {
        vec!["Mr.", "Dr.", "Prof."]
    }

    pub fn title_female(&self) -> Vec<&'a str> {
        vec!["Mrs.", "Ms.", "Miss", "Dr.", "Prof."]
    }
}
