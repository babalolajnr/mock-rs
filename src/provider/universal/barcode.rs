use crate::{calculator::ean, provider::base::BaseTrait};

pub trait BarcodeTrait: BaseTrait {
    /// Get random EAN
    fn ean(length: u8) -> String {
        let code = Self::numerify(Some(&str::repeat("#", (length - 1).into())));
        format!("{}{}", code, ean::checksum(&code))
    }

    /// Get random EAN-13 barcode
    fn ean13() -> String {
        Self::ean(13)
    }

    /// Get random EAN-8 barcode
    fn ean8() -> String {
        Self::ean(8)
    }

    /// Get random ISBN-10
    fn isbn10() -> String {
        // let code = .numerify(Some(&str::repeat("#", (9).into())));
        todo!()
    }

    /// Get random ISBN-13
    fn isbn13() -> String {
        todo!()
    }
}
