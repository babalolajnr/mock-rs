struct Generator<'a> {
    providers: Vec<&'a str>,
    formatters: Vec<&'a str>,
}

impl<'a> Generator<'a> {
    pub fn parse(string: &str) {}

    // pub fn get_formatter(self, format: &'a str) -> &'a str {
    //     if self.formatters.contains(&format) {
    //         return format
    //     }else if() {
            
    //     }
    // }

    pub fn add_providers(&mut self, provider: &'a str) {
        let provider = [provider];
        &self.providers.splice(1..1, provider.iter().cloned());
    }

    pub fn format(format: &str, args: &[&str]) {}
}
