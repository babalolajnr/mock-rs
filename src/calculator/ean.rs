///Computes the checksum of an EAN number
fn checksum(digits: &str) -> u8 {
    let sequence = if (digits.len() + 1) == 8 {
        [3, 1]
    } else {
        [1, 3]
    };

    let mut sums = 0;

    let mut index = 0;

    for digit in digits.chars() {
        sums += digit.to_string().parse::<u8>().unwrap() * sequence[index % 2];
        index += 1
    }

    (10 - sums % 10) % 10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_checksum() {
        let checksum = checksum("235469852146");
        assert_eq!(checksum, 9);
    }
}
