fn checksum(number: &str) -> u32 {
    let mut sum = 0;
    for (i, c) in number.chars().rev().enumerate() {
        let digit = c.to_digit(10).unwrap();
        let weight = if i % 2 == 0 { digit } else { digit * 2 };
        sum += weight / 10;
        sum += weight % 10;
    }
    sum % 10
}

#[cfg(test)]
mod tests {
    use super::checksum;

    #[test]
    fn test_checksum() {
        let checksum = checksum("0787686577456576");
        println!("{}", checksum)
    }
}
