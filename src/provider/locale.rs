pub(crate) trait Locale {
    fn name<'a>(&self) -> &'a str;
}
