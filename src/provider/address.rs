use crate::helpers::base::random_index;


pub trait AddressTrait {
    fn city_suffix(&self) -> String;

    fn street_suffix(&self) -> String;

    fn building_number(&self) -> String;

    fn city(&self) -> String;

    fn street_name(&self) -> String;

    fn street_address(&self) -> String;

    fn postcode(&self) -> String;

    fn address(&self) -> String;

    fn latitude() -> f64 {
        let latitude = rand::random::<f64>() * 180.0 - 90.0;
        format!("{:.6}", latitude).parse::<f64>().unwrap()
    }

    fn longitude() -> f64 {
        let longitude = rand::random::<f64>() * 360.0 - 180.0;
        format!("{:.6}", longitude).parse::<f64>().unwrap()
    }

    /// Returns (latitude, longitude)
    fn local_cordinates() -> (f64, f64) {
        (Self::latitude(), Self::longitude())
    }

    fn get_format(formats: Vec<String>) -> String {
        let random_index = random_index(&formats);
        formats[random_index].to_string()
    }
}
