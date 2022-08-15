# mock-rs
A Rust crate to generate fake data inspired by FakerPHP. The crate is still under development and it is not stable yet. So a lot of thing might change in the future.

## Structure
The crate is divided into providers. A provider is a module that represents a particular locale. A provider contains sub-providers for that particular locale The available locales right now are: `en_us` and `universal`. Many more are going to be added in the future.

## Examples
### `en_us` Person provider
```rust
use mock_rs::{en_us::Person, PersonTrait, Gender};

// Initialize `en_us` person
let en_us_person = Person::new(); 

// Get random en_us male_name
let male_name = en_us_person.name(Some(Gender::Male));
println!("{}", male_name);

// Get random en_us female_name
let female_male_name = en_us_person.name(Some(Gender::Female));
println!("{}", female_male_name);


```
