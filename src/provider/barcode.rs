use crate::calculator::ean;

use super::base::Base;

pub trait Barcode: Base {
    fn ean(&self, length: u8) -> String {
        let code = &self.numerify(Some(&str::repeat("#", (length - 1).into())));

        format!("{}{}", code, ean::checksum(code))
    }

    /// Get random EAN-13 barcode
    fn ean13(&self) -> String {
        self.ean(13)
    }

    /// Get random EAN-8 barcode
    fn ean8(&self) -> String {
        self.ean(8)
    }

    
}
