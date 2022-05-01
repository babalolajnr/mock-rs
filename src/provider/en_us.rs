pub mod person;
pub mod address;

pub struct EnUS<'a> {
    address: address::Address<'a>,
    person: person::Person,
}
