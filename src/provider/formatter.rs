pub trait Formatter {
    fn name<'a>(&self) -> &'a str;
}
