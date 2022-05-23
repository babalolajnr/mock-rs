pub fn checksum(number: &str) -> u32 {
    let mut sum = 0;
    for (i, c) in number.chars().rev().enumerate() {
        let digit = c.to_digit(10).unwrap();
        let weight = if i % 2 == 0 { digit } else { digit * 2 };
        sum += weight / 10;
        sum += weight % 10;
    }
    sum % 10
}

pub fn compute_check_digit(partial_number: &str) -> String {
    let check_digit = 10 - checksum(format!("{}{}", partial_number, "0").as_str());

    if check_digit == 0 {
        return 0.to_string();
    }

    format!("{}", check_digit)
}

#[cfg(test)]
mod tests {
    use super::{checksum, compute_check_digit};

    #[test]
    fn test_checksum() {
        let checksum = checksum("234512");
        assert_eq!(checksum, 4);
    }

    #[test]
    fn test_compute_check_digit() {
        let check_digit = compute_check_digit("234512");
        assert_eq!(check_digit, "2");
    }
}
