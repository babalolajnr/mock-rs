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
}
