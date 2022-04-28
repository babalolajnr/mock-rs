use super::Provider;

enum Gender {
    Male,
    Female,
}
pub(crate) struct Person {
    gender: Gender,
}

trait Format<'a> {
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
}

impl<'a> Format<'a> for Person {}

impl Provider for Person {
    fn name<'a>(&self) -> &'a str {
        "person"
    }
}

impl<'a> Person {
    pub fn first_name() -> &'a str {
        "John"
    }
    pub fn new() -> Self {
        Self {
            gender: Gender::Male,
        }
    }
}

