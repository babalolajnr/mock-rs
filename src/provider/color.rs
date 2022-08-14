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

    /// Generate random  safe hex color
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
}

#[cfg(test)]
mod tests{
    use super::*;
    struct Test;
    impl<'a> ColorTrait<'a> for Test {}

    #[test]
    fn test_hex_color() {
        let hex_color = Test::hex_color();
        assert_eq!(hex_color.len(), 7)
    }
}