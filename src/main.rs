
// use mock_rs::{Color, Barcode, BarcodeTrait, ColorTrait};

use mock_rs::{en_us::Person, PersonTrait, Gender};


fn main() {
    // let color = Color::hex_color();
    // println!("{}", color);
    // let barcode = Barcode::
    let en_us_person = Person::new(); 
    let male_name = en_us_person.name(Some(Gender::Male));
    println!("{}", male_name);

    
}
