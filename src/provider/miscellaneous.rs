use std::cmp;

use rand::Rng;

pub struct Miscellaneous{}

impl Miscellaneous{
    pub fn boolean(chance_of_getting_true: Option<u8>) -> bool {
        todo!()
    }

    /// Returns a random number between int_1 and int_2 (any order)
    /// 
    /// int int_1 default to 0
    /// int int_2 defaults to 32 bit max integer, ie 2147483647
    /// 
    /// example 79907610
    ///
    pub fn number_between(int_1: Option<u32>, int_2: Option<u32>) -> u32 {
        let int_1 = int_1.unwrap_or(0);
        let int_2 = int_2.unwrap_or(2147483647);

        let min = cmp::min(int_1, int_2);
        let max = cmp::max(int_1, int_2);

        rand::thread_rng().gen_range(min..max)
    }
}