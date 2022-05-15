use crate::calculator::ean;

use super::base::Base;

pub struct Barcode {}

impl Base for Barcode {}

impl Barcode {
    pub fn ean(length: u8) -> String {
        let code = Self::numerify(Some(&str::repeat("#", (length - 1).into())));
        format!("{}{}", code, ean::checksum(&code))
    }

    /// Get random EAN-13 barcode
    pub fn ean13() -> String {
        Self::ean(13)
    }

    /// Get random EAN-8 barcode
    pub fn ean8() -> String {
        Self::ean(8)
    }

    /// Get random ISBN-10
    pub fn isbn10() -> String {
        // let code = .numerify(Some(&str::repeat("#", (9).into())));
        todo!()
    }

    /// Get random ISBN-13
    pub fn isbn13() -> String {
        todo!()
    }
}

