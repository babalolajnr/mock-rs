use crate::provider::address::AddressTrait;
use crate::provider::base::BaseTrait;
use crate::provider::person::PersonTrait;

use super::person::Person as PersonProvider;

/// EN_US address provider generates random address related data
pub struct Address<'a> {
    city_prefix: Vec<&'a str>,
    city_suffix: Vec<&'a str>,
    building_number: Vec<&'a str>,
    street_suffix: Vec<&'a str>,
    postcode: Vec<&'a str>,
    state: Vec<&'a str>,
    state_abbr: Vec<&'a str>,
    country: Vec<&'a str>,
    secondary_address_formats: Vec<&'a str>,
    person: Box<dyn PersonTrait>,
}

impl Address<'_> {
    pub fn new() -> Self {
        Self {
            city_prefix: vec!["North", "East", "West", "South", "New", "Lake", "Port"],
            city_suffix: vec![
                "town", "ton", "land", "ville", "berg", "burgh", "borough", "bury", "view", "port",
                "mouth", "stad", "furt", "chester", "mouth", "fort", "haven", "side", "shire",
            ],
            building_number: vec!["%####", "%###", "%##"],
            street_suffix: vec![
                "Alley",
                "Avenue",
                "Branch",
                "Bridge",
                "Brook",
                "Brooks",
                "Burg",
                "Burgs",
                "Bypass",
                "Camp",
                "Canyon",
                "Cape",
                "Causeway",
                "Center",
                "Centers",
                "Circle",
                "Circles",
                "Cliff",
                "Cliffs",
                "Club",
                "Common",
                "Corner",
                "Corners",
                "Course",
                "Court",
                "Courts",
                "Cove",
                "Coves",
                "Creek",
                "Crescent",
                "Crest",
                "Crossing",
                "Crossroad",
                "Curve",
                "Dale",
                "Dam",
                "Divide",
                "Drive",
                "Drive",
                "Drives",
                "Estate",
                "Estates",
                "Expressway",
                "Extension",
                "Extensions",
                "Fall",
                "Falls",
                "Ferry",
                "Field",
                "Fields",
                "Flat",
                "Flats",
                "Ford",
                "Fords",
                "Forest",
                "Forge",
                "Forges",
                "Fork",
                "Forks",
                "Fort",
                "Freeway",
                "Garden",
                "Gardens",
                "Gateway",
                "Glen",
                "Glens",
                "Green",
                "Greens",
                "Grove",
                "Groves",
                "Harbor",
                "Harbors",
                "Haven",
                "Heights",
                "Highway",
                "Hill",
                "Hills",
                "Hollow",
                "Inlet",
                "Inlet",
                "Island",
                "Island",
                "Islands",
                "Islands",
                "Isle",
                "Isle",
                "Junction",
                "Junctions",
                "Key",
                "Keys",
                "Knoll",
                "Knolls",
                "Lake",
                "Lakes",
                "Land",
                "Landing",
                "Lane",
                "Light",
                "Lights",
                "Loaf",
                "Lock",
                "Locks",
                "Locks",
                "Lodge",
                "Lodge",
                "Loop",
                "Mall",
                "Manor",
                "Manors",
                "Meadow",
                "Meadows",
                "Mews",
                "Mill",
                "Mills",
                "Mission",
                "Mission",
                "Motorway",
                "Mount",
                "Mountain",
                "Mountain",
                "Mountains",
                "Mountains",
                "Neck",
                "Orchard",
                "Oval",
                "Overpass",
                "Park",
                "Parks",
                "Parkway",
                "Parkways",
                "Pass",
                "Passage",
                "Path",
                "Pike",
                "Pine",
                "Pines",
                "Place",
                "Plain",
                "Plains",
                "Plains",
                "Plaza",
                "Plaza",
                "Point",
                "Points",
                "Port",
                "Port",
                "Ports",
                "Ports",
                "Prairie",
                "Prairie",
                "Radial",
                "Ramp",
                "Ranch",
                "Rapid",
                "Rapids",
                "Rest",
                "Ridge",
                "Ridges",
                "River",
                "Road",
                "Road",
                "Roads",
                "Roads",
                "Route",
                "Row",
                "Rue",
                "Run",
                "Shoal",
                "Shoals",
                "Shore",
                "Shores",
                "Skyway",
                "Spring",
                "Springs",
                "Springs",
                "Spur",
                "Spurs",
                "Square",
                "Square",
                "Squares",
                "Squares",
                "Station",
                "Station",
                "Stravenue",
                "Stravenue",
                "Stream",
                "Stream",
                "Street",
                "Street",
                "Streets",
                "Summit",
                "Summit",
                "Terrace",
                "Throughway",
                "Trace",
                "Track",
                "Trafficway",
                "Trail",
                "Trail",
                "Tunnel",
                "Tunnel",
                "Turnpike",
                "Turnpike",
                "Underpass",
                "Union",
                "Unions",
                "Valley",
                "Valleys",
                "Via",
                "Viaduct",
                "View",
                "Views",
                "Village",
                "Village",
                "Villages",
                "Ville",
                "Vista",
                "Vista",
                "Walk",
                "Walks",
                "Wall",
                "Way",
                "Ways",
                "Well",
                "Wells",
            ],
            postcode: vec!["#####", "#####-####"],
            state: vec![
                "Alabama",
                "Alaska",
                "Arizona",
                "Arkansas",
                "California",
                "Colorado",
                "Connecticut",
                "Delaware",
                "District of Columbia",
                "Florida",
                "Georgia",
                "Hawaii",
                "Idaho",
                "Illinois",
                "Indiana",
                "Iowa",
                "Kansas",
                "Kentucky",
                "Louisiana",
                "Maine",
                "Maryland",
                "Massachusetts",
                "Michigan",
                "Minnesota",
                "Mississippi",
                "Missouri",
                "Montana",
                "Nebraska",
                "Nevada",
                "New Hampshire",
                "New Jersey",
                "New Mexico",
                "New York",
                "North Carolina",
                "North Dakota",
                "Ohio",
                "Oklahoma",
                "Oregon",
                "Pennsylvania",
                "Rhode Island",
                "South Carolina",
                "South Dakota",
                "Tennessee",
                "Texas",
                "Utah",
                "Vermont",
                "Virginia",
                "Washington",
                "West Virginia",
                "Wisconsin",
                "Wyoming",
            ],
            state_abbr: vec![
                "AK", "AL", "AR", "AZ", "CA", "CO", "CT", "DC", "DE", "FL", "GA", "HI", "IA", "ID",
                "IL", "IN", "KS", "KY", "LA", "MA", "MD", "ME", "MI", "MN", "MO", "MS", "MT", "NC",
                "ND", "NE", "NH", "NJ", "NM", "NV", "NY", "OH", "OK", "OR", "PA", "RI", "SC", "SD",
                "TN", "TX", "UT", "VA", "VT", "WA", "WI", "WV", "WY",
            ],
            country: vec![
                "Afghanistan",
                "Albania",
                "Algeria",
                "American Samoa",
                "Andorra",
                "Angola",
                "Anguilla",
                "Antarctica (the territory South of 60 deg S)",
                "Antigua and Barbuda",
                "Argentina",
                "Armenia",
                "Aruba",
                "Australia",
                "Austria",
                "Azerbaijan",
                "Bahamas",
                "Bahrain",
                "Bangladesh",
                "Barbados",
                "Belarus",
                "Belgium",
                "Belize",
                "Benin",
                "Bermuda",
                "Bhutan",
                "Bolivia",
                "Bosnia and Herzegovina",
                "Botswana",
                "Bouvet Island (Bouvetoya)",
                "Brazil",
                "British Indian Ocean Territory (Chagos Archipelago)",
                "British Virgin Islands",
                "Brunei Darussalam",
                "Bulgaria",
                "Burkina Faso",
                "Burundi",
                "Cambodia",
                "Cameroon",
                "Canada",
                "Cape Verde",
                "Cayman Islands",
                "Central African Republic",
                "Chad",
                "Chile",
                "China",
                "Christmas Island",
                "Cocos (Keeling) Islands",
                "Colombia",
                "Comoros",
                "Congo",
                "Cook Islands",
                "Costa Rica",
                "Cote d\"Ivoire",
                "Croatia",
                "Cuba",
                "Cyprus",
                "Czech Republic",
                "Denmark",
                "Djibouti",
                "Dominica",
                "Dominican Republic",
                "Ecuador",
                "Egypt",
                "El Salvador",
                "Equatorial Guinea",
                "Eritrea",
                "Estonia",
                "Ethiopia",
                "Faroe Islands",
                "Falkland Islands (Malvinas)",
                "Fiji",
                "Finland",
                "France",
                "French Guiana",
                "French Polynesia",
                "French Southern Territories",
                "Gabon",
                "Gambia",
                "Georgia",
                "Germany",
                "Ghana",
                "Gibraltar",
                "Greece",
                "Greenland",
                "Grenada",
                "Guadeloupe",
                "Guam",
                "Guatemala",
                "Guernsey",
                "Guinea",
                "Guinea-Bissau",
                "Guyana",
                "Haiti",
                "Heard Island and McDonald Islands",
                "Holy See (Vatican City State)",
                "Honduras",
                "Hong Kong",
                "Hungary",
                "Iceland",
                "India",
                "Indonesia",
                "Iran",
                "Iraq",
                "Ireland",
                "Isle of Man",
                "Israel",
                "Italy",
                "Jamaica",
                "Japan",
                "Jersey",
                "Jordan",
                "Kazakhstan",
                "Kenya",
                "Kiribati",
                "Korea",
                "Korea",
                "Kuwait",
                "Kyrgyz Republic",
                "Lao People\"s Democratic Republic",
                "Latvia",
                "Lebanon",
                "Lesotho",
                "Liberia",
                "Libyan Arab Jamahiriya",
                "Liechtenstein",
                "Lithuania",
                "Luxembourg",
                "Macao",
                "Macedonia",
                "Madagascar",
                "Malawi",
                "Malaysia",
                "Maldives",
                "Mali",
                "Malta",
                "Marshall Islands",
                "Martinique",
                "Mauritania",
                "Mauritius",
                "Mayotte",
                "Mexico",
                "Micronesia",
                "Moldova",
                "Monaco",
                "Mongolia",
                "Montenegro",
                "Montserrat",
                "Morocco",
                "Mozambique",
                "Myanmar",
                "Namibia",
                "Nauru",
                "Nepal",
                "Netherlands Antilles",
                "Netherlands",
                "New Caledonia",
                "New Zealand",
                "Nicaragua",
                "Niger",
                "Nigeria",
                "Niue",
                "Norfolk Island",
                "Northern Mariana Islands",
                "Norway",
                "Oman",
                "Pakistan",
                "Palau",
                "Palestinian Territories",
                "Panama",
                "Papua New Guinea",
                "Paraguay",
                "Peru",
                "Philippines",
                "Pitcairn Islands",
                "Poland",
                "Portugal",
                "Puerto Rico",
                "Qatar",
                "Reunion",
                "Romania",
                "Russian Federation",
                "Rwanda",
                "Saint Barthelemy",
                "Saint Helena",
                "Saint Kitts and Nevis",
                "Saint Lucia",
                "Saint Martin",
                "Saint Pierre and Miquelon",
                "Saint Vincent and the Grenadines",
                "Samoa",
                "San Marino",
                "Sao Tome and Principe",
                "Saudi Arabia",
                "Senegal",
                "Serbia",
                "Seychelles",
                "Sierra Leone",
                "Singapore",
                "Slovakia (Slovak Republic)",
                "Slovenia",
                "Solomon Islands",
                "Somalia",
                "South Africa",
                "South Georgia and the South Sandwich Islands",
                "Spain",
                "Sri Lanka",
                "Sudan",
                "Suriname",
                "Svalbard & Jan Mayen Islands",
                "Swaziland",
                "Sweden",
                "Switzerland",
                "Syrian Arab Republic",
                "Taiwan",
                "Tajikistan",
                "Tanzania",
                "Thailand",
                "Timor-Leste",
                "Togo",
                "Tokelau",
                "Tonga",
                "Trinidad and Tobago",
                "Tunisia",
                "Turkey",
                "Turkmenistan",
                "Turks and Caicos Islands",
                "Tuvalu",
                "Uganda",
                "Ukraine",
                "United Arab Emirates",
                "United Kingdom",
                "United States of America",
                "United States Minor Outlying Islands",
                "United States Virgin Islands",
                "Uruguay",
                "Uzbekistan",
                "Vanuatu",
                "Venezuela",
                "Vietnam",
                "Wallis and Futuna",
                "Western Sahara",
                "Yemen",
                "Zambia",
                "Zimbabwe",
            ],
            secondary_address_formats: vec!["Apt. ###", "Suite ###"],
            person: Box::new(PersonProvider::new()),
        }
    }

    pub fn state(&self) -> String {
        Self::random_element(&self.state).to_string()
    }

    pub fn state_abbr(&self) -> String {
        Self::random_element(&self.state_abbr).to_string()
    }

    pub fn country(&self) -> String {
        Self::random_element(&self.country).to_string()
    }

    pub fn secondary_address(&self) -> String {
        Self::numerify(Some(Self::random_element(&self.secondary_address_formats)))
    }

    pub fn city_prefix(&self) -> String {
        Self::random_element(&self.city_prefix).to_string()
    }
}

impl BaseTrait for Address<'_> {}
impl AddressTrait for Address<'_> {
    fn city_suffix(&self) -> String {
        Self::random_element(&self.city_suffix).to_string()
    }

    fn street_suffix(&self) -> String {
        Self::random_element(&self.street_suffix).to_string()
    }

    fn building_number(&self) -> String {
        Self::bothify(Some(Self::random_element(&self.building_number))).to_string()
    }

    fn city(&self) -> String {
        let formats = vec![
            format!(
                "{} {}{}",
                &self.city_prefix(),
                &self.person.first_name(None),
                &self.city_suffix()
            ),
            format!("{} {}", &self.city_prefix(), &self.person.first_name(None)),
            format!("{} {}", &self.person.first_name(None), &self.city_suffix()),
            format!("{} {}", &self.person.last_name(), &self.city_suffix()),
        ];

        Self::get_format(formats)
    }

    fn street_name(&self) -> String {
        let formats = vec![
            format!(
                "{} {}",
                &self.person.first_name(None),
                &self.street_suffix()
            ),
            format!("{} {}", &self.person.last_name(), &self.street_suffix()),
        ];

        Self::get_format(formats)
    }

    fn street_address(&self) -> String {
        let formats = vec![
            format!("{} {}", &self.building_number(), &self.street_name()),
            format!(
                "{} {} {}",
                &self.building_number(),
                &self.street_name(),
                &self.secondary_address()
            ),
        ];

        Self::get_format(formats)
    }

    fn address(&self) -> String {
        let formats = vec![
            format!("{}\n{}", self.street_address(), self.city()),
            format!("{} {}", self.state_abbr(), self.postcode()),
        ];

        Self::get_format(formats)
    }

    fn postcode(&self) -> String {
        Self::bothify(Some(Self::random_element(&self.postcode))).to_string()
    }
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn city() {
        let address = Address::new();
        let city = address.city();
        assert!(city.len() > 1);
    }

    #[test]
    fn street_name() {
        let address = Address::new();
        let street_name = address.street_name();
        assert!(street_name.len() > 1);
    }

    #[test]
    fn street_address() {
        let address = Address::new();
        let street_address = address.street_address();
        assert!(street_address.len() > 1);
    }

    #[test]
    fn address() {
        let address = Address::new().address();
        assert!(address.len() > 1);
    }

    #[test]
    fn latitude() {
        let latitude = Address::latitude();
        assert!(latitude >= -90.0);
        assert!(latitude <= 90.0);
    }

    #[test]
    fn longitude() {
        let longitude = Address::longitude();
        assert!(longitude >= -180.0);
        assert!(longitude <= 180.0);
    }

    #[test]
    fn local_cordinates() {
        let (latitude, longitude) = Address::local_cordinates();
        assert!(latitude >= -90.0);
        assert!(latitude <= 90.0);
        assert!(longitude >= -180.0);
        assert!(longitude <= 180.0);
    }
}
