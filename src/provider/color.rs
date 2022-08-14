use crate::helpers::miscellaneous::number_between;

pub trait ColorTrait<'a> {
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
    /// let hex_color = hex_color();
    /// assert_eq!(hex_color.len(), 7);
    /// ```
    fn hex_color() -> String {
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
    /// let safe_hex_color = safe_hex_color();
    /// assert_eq!(safe_hex_color.len(), 7);
    /// ```
    fn safe_hex_color() -> String {
        let rand_number = number_between(Some(1), Some(255));
        let hex = format!("{rand_number:x}");
        format!("#{:0>6}", hex)
    }

    /// Generate `RGB` color as an array
    fn rgb_color_as_array() -> [String; 3] {
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
    fn rgb_color() -> String {
        format!("{}", Self::rgb_color_as_array().join(","))
    }

    fn rgb_css_color() -> String {
        format!("rgb({})", Self::rgb_color_as_array().join(","))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    struct Test;
    impl<'a> ColorTrait<'a> for Test {}

    #[test]
    fn test_hex_color() {
        let hex_color = Test::hex_color();
        assert_eq!(hex_color.len(), 7)
    }

    #[test]
    fn test_safe_hex_color() {
        let safe_hex_color = Test::safe_hex_color();
        assert_eq!(safe_hex_color.len(), 7)
    }

    #[test]
    fn test_rgb_color_as_array() {
        let rgb_color = Test::rgb_color_as_array();
        assert_eq!(rgb_color.len(), 3);
    }

    #[test]
    fn test_rgb_css_color() {
        let rgb_color = Test::rgb_css_color();
        assert!(rgb_color.len() > 0);
        assert!(rgb_color.contains("rgb"));
    }

    #[test]
    fn test_rgb_color() {
        let rgb_color = Test::rgb_color();
        assert!(rgb_color.len() > 0);
        println!("{}", rgb_color)
    }
}
