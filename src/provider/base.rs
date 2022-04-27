use rand::{thread_rng, Rng};
pub trait Base {
    fn generate_random_index<T>(arr: &[T]) -> usize {
        let random_index = thread_rng().gen_range(1..arr.len());
        random_index
    }

    fn random_element<'a, T: ?Sized>(arr: &'a Vec<&'a T>) -> &'a T {
        let random_index = Self::generate_random_index(&arr);
        &arr[random_index]
    }

    /// Generates a random string that matches the
    /// length of the given string containing "#"s
    fn numerify(string: Option<&str>) -> String {
        let string = string.unwrap_or("###");

        let charset: &[u8] = b"0123456789";
        let mut rng = rand::thread_rng();

        let random_string: String = (0..string.len())
            .map(|_| {
                let idx = rng.gen_range(0..charset.len());
                charset[idx] as char
            })
            .collect();

        random_string
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn numerify_works() {
        use super::*;

        let string = "####";
        struct Test;
        impl Base for Test {}

        let result = Test::numerify(Some(string));
        assert_eq!(result.len(), string.len());
    }
}
