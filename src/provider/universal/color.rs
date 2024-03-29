use crate::helpers::{base::random_element, miscellaneous::number_between};
use rand::Rng;

/// `Color` struct contains methods relating to random color generation
pub struct Color;

impl<'a> Color {
    fn safe_color_names() -> [&'a str; 15] {
        [
            "black", "maroon", "green", "navy", "olive", "purple", "teal", "lime", "blue",
            "silver", "gray", "yellow", "fuchsia", "aqua", "white",
        ]
    }

    fn all_color_names() -> [&'a str; 141] {
        [
            "AliceBlue",
            "AntiqueWhite",
            "Aqua",
            "Aquamarine",
            "Azure",
            "Beige",
            "Bisque",
            "Black",
            "BlanchedAlmond",
            "Blue",
            "BlueViolet",
            "Brown",
            "BurlyWood",
            "CadetBlue",
            "Chartreuse",
            "Chocolate",
            "Coral",
            "CornflowerBlue",
            "Cornsilk",
            "Crimson",
            "Cyan",
            "DarkBlue",
            "DarkCyan",
            "DarkGoldenRod",
            "DarkGray",
            "DarkGreen",
            "DarkKhaki",
            "DarkMagenta",
            "DarkOliveGreen",
            "Darkorange",
            "DarkOrchid",
            "DarkRed",
            "DarkSalmon",
            "DarkSeaGreen",
            "DarkSlateBlue",
            "DarkSlateGray",
            "DarkTurquoise",
            "DarkViolet",
            "DeepPink",
            "DeepSkyBlue",
            "DimGray",
            "DimGrey",
            "DodgerBlue",
            "FireBrick",
            "FloralWhite",
            "ForestGreen",
            "Fuchsia",
            "Gainsboro",
            "GhostWhite",
            "Gold",
            "GoldenRod",
            "Gray",
            "Green",
            "GreenYellow",
            "HoneyDew",
            "HotPink",
            "IndianRed",
            "Indigo",
            "Ivory",
            "Khaki",
            "Lavender",
            "LavenderBlush",
            "LawnGreen",
            "LemonChiffon",
            "LightBlue",
            "LightCoral",
            "LightCyan",
            "LightGoldenRodYellow",
            "LightGray",
            "LightGreen",
            "LightPink",
            "LightSalmon",
            "LightSeaGreen",
            "LightSkyBlue",
            "LightSlateGray",
            "LightSteelBlue",
            "LightYellow",
            "Lime",
            "LimeGreen",
            "Linen",
            "Magenta",
            "Maroon",
            "MediumAquaMarine",
            "MediumBlue",
            "MediumOrchid",
            "MediumPurple",
            "MediumSeaGreen",
            "MediumSlateBlue",
            "MediumSpringGreen",
            "MediumTurquoise",
            "MediumVioletRed",
            "MidnightBlue",
            "MintCream",
            "MistyRose",
            "Moccasin",
            "NavajoWhite",
            "Navy",
            "OldLace",
            "Olive",
            "OliveDrab",
            "Orange",
            "OrangeRed",
            "Orchid",
            "PaleGoldenRod",
            "PaleGreen",
            "PaleTurquoise",
            "PaleVioletRed",
            "PapayaWhip",
            "PeachPuff",
            "Peru",
            "Pink",
            "Plum",
            "PowderBlue",
            "Purple",
            "Red",
            "RosyBrown",
            "RoyalBlue",
            "SaddleBrown",
            "Salmon",
            "SandyBrown",
            "SeaGreen",
            "SeaShell",
            "Sienna",
            "Silver",
            "SkyBlue",
            "SlateBlue",
            "SlateGray",
            "Snow",
            "SpringGreen",
            "SteelBlue",
            "Tan",
            "Teal",
            "Thistle",
            "Tomato",
            "Turquoise",
            "Violet",
            "Wheat",
            "White",
            "WhiteSmoke",
            "Yellow",
            "YellowGreen",
        ]
    }

    /// Generate random Hex color
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use mock_rs::Color;
    ///
    /// let hex_color = Color::hex_color();
    /// assert_eq!(hex_color.len(), 7);
    /// ```
    pub fn hex_color() -> String {
        let rand_number = number_between(Some(1), Some(16777215));
        let hex = format!("{rand_number:x}");
        format!("#{:0>6}", hex)
    }

    /// Generate random safe hex color
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use mock_rs::Color;
    ///
    /// let safe_hex_color = Color::safe_hex_color();
    /// assert_eq!(safe_hex_color.len(), 7);
    /// ```
    pub fn safe_hex_color() -> String {
        let rand_number = number_between(Some(1), Some(255));
        let hex = format!("{rand_number:x}");
        format!("#{:0>6}", hex)
    }

    /// Generate `RGB` color as an array
    /// e.g `[121,121,121]`
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use mock_rs::Color;
    ///
    /// let rgb_color_as_array = Color::rgb_color_as_array();
    /// assert_eq!(rgb_color_as_array.len(), 3);
    /// ```
    pub fn rgb_color_as_array() -> [String; 3] {
        let color = Self::hex_color();

        let r = color.chars().skip(1).take(2).collect::<String>();
        let g = color.chars().skip(3).take(2).collect::<String>();
        let b = color.chars().skip(5).take(2).collect::<String>();

        [
            format!("{}", i64::from_str_radix(&r, 16).unwrap()),
            format!("{}", i64::from_str_radix(&g, 16).unwrap()),
            format!("{}", i64::from_str_radix(&b, 16).unwrap()),
        ]
    }

    /// Generate `RGB` color string
    /// e.g `121,121,121`
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use mock_rs::Color;
    ///
    /// let rgb_color = Color::rgb_color();
    /// ```
    pub fn rgb_color() -> String {
        format!("{}", Self::rgb_color_as_array().join(","))
    }

    /// Generate `RGB` css color string.
    /// e.g `rgb(121,121,121)`
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use mock_rs::Color;
    ///
    /// let rgb_css_color = Color::rgb_css_color();
    /// ```
    pub fn rgb_css_color() -> String {
        format!("rgb({})", Self::rgb_color_as_array().join(","))
    }

    /// Generate safe color name
    /// e.g `white`
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use mock_rs::Color;
    ///
    /// let safe_color_name = Color::safe_color_name();
    /// ```
    pub fn safe_color_name() -> String {
        let safe_color_names_vec = &Self::safe_color_names().to_vec();
        random_element(safe_color_names_vec).to_string()
    }

    /// Generate color name
    /// e.g `Azure`
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use mock_rs::Color;
    ///
    /// let color_name = Color::color_name();
    /// ```
    pub fn color_name() -> String {
        let all_color_names_vec = &Self::all_color_names().to_vec();
        random_element(all_color_names_vec).to_string()
    }

    /// Generate random hsl color
    /// e.g `340,50,20`
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use mock_rs::Color;
    ///
    /// let hsl_color = Color::hsl_color();
    /// ```
    pub fn hsl_color() -> String {
        format!(
            "{},{},{}",
            number_between(Some(0), Some(360)),
            number_between(Some(0), Some(100)),
            number_between(Some(0), Some(100))
        )
    }

    /// Generate random hsl color array
    /// e.g `[340,50,20]`
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use mock_rs::Color;
    ///
    /// let hsl_color_as_array = Color::hsl_color_as_array();
    /// ```
    pub fn hsl_color_as_array() -> [u32; 3] {
        [
            number_between(Some(0), Some(360)),
            number_between(Some(0), Some(100)),
            number_between(Some(0), Some(100)),
        ]
    }

    /// Generate random `rgba` css color
    /// e.g `rgba(0,255,122,0.8)`
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use mock_rs::Color;
    ///
    /// let rgba_css_color = Color::rgba_css_color();
    /// ```
    pub fn rgba_css_color() -> String {
        format!(
            "rgba({},{:.1})",
            Self::rgb_color(),
            rand::thread_rng().gen::<f32>()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_color() {
        let hex_color = Color::hex_color();
        assert_eq!(hex_color.len(), 7)
    }

    #[test]
    fn test_safe_hex_color() {
        let safe_hex_color = Color::safe_hex_color();
        assert_eq!(safe_hex_color.len(), 7)
    }

    #[test]
    fn test_rgb_color_as_array() {
        let rgb_color = Color::rgb_color_as_array();
        assert_eq!(rgb_color.len(), 3);
    }

    #[test]
    fn test_rgb_css_color() {
        let rgb_color = Color::rgb_css_color();
        assert!(rgb_color.len() > 0);
        assert!(rgb_color.contains("rgb"));
    }

    #[test]
    fn test_rgb_color() {
        let rgb_color = Color::rgb_color();
        assert!(rgb_color.len() > 0);
    }

    #[test]
    fn test_rgba_css_color() {
        let rgba_color = Color::rgba_css_color();
        assert!(rgba_color.len() > 0);
        assert!(rgba_color.contains("rgba"));
    }
}
