pub trait CompanyTrait {
    /// Example: 'Acme Ltd'
    fn company(&self) -> String;

    fn company_suffix(&self) -> String;

    fn job_title(&self) -> String;
}
