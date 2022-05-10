use super::base::Base;

pub trait Address: Base {
    fn city_suffix(&self) -> String;

    fn street_suffix(&self) -> String;

    fn building_number(&self) -> String;

    fn city(&self) -> String;

    fn street_name(&self) -> String;

    fn street_address(&self) -> String;
    
    fn postcode(&self) -> String;
    
    fn address(&self) -> String;

    fn latitude(&self) -> f64 {
        let latitude = rand::random::<f64>() * 180.0 - 90.0;
        format!("{:.6}", latitude).parse::<f64>().unwrap()
    }

    fn longitude(&self) -> f64 {
        let longitude = rand::random::<f64>() * 360.0 - 180.0;
        format!("{:.6}", longitude).parse::<f64>().unwrap()
    }

    /// Returns (latitude, longitude)
    fn local_cordinates(&self) -> (f64, f64) {
        (self.latitude(), self.longitude())
    }

    fn get_format(&self, formats: Vec<String>) -> String {
        let random_index = self.random_index(&formats);
        formats[random_index].to_string()
    }
}
