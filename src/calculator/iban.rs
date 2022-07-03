/// Calculates mod97 on a numeric string
pub fn mod97(number: &str) -> u32 {
    let mut checksum = number.chars().nth(0).unwrap().to_digit(10).unwrap();

    for i in number.to_string()[1..].chars() {
        checksum = (10 * checksum + i.to_digit(10).unwrap()) % 97;
    }

    checksum
}

/// Generates IBAN checksum
pub fn checksum(iban: &str) -> String {
    let first_part = &iban.to_string()[4..];
    let last_part = &iban.to_string()[..2];

    let check_string = format!("{}{}00", first_part, last_part);

    let check_string = check_string
        .chars()
        .map(|i| {
            if i.is_numeric() {
                i.to_string()
            } else {
                alpha_to_number(i).to_string()
            }
        })
        .collect::<String>();

    let checksum = 98 - mod97(&check_string);
    format!("{:0>2}", checksum)
}

/// Converts letter to number
fn alpha_to_number(character: char) -> u32 {
    (character as u32) - 55
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_mod97() {
        assert_eq!(super::mod97("5989800900"), 72);
    }

    #[test]
    fn checksum() {
        assert_eq!("09", super::checksum("MIDL40051512345674GB11"));
    }
}
