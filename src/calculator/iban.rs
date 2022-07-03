pub fn mod97(number: &str) -> u32 {
    let mut checksum = number.chars().nth(0).unwrap().to_digit(10).unwrap();

    for i in number.to_string()[1..].chars() {
        checksum = (10 * checksum + i.to_digit(10).unwrap()) % 97;
    }

    checksum
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_mod97() {
        assert_eq!(super::mod97("5989800900"), 72);
    }
}
