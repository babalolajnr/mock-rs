use crate::{calculator::ean, helpers::base::numerify};

/// `Barcode` struct contains methods for generating different barcode formats
pub struct Barcode;

impl Barcode {
    /// Get random EAN
    /// 
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use mock_rs::Barcode;
    ///
    /// let ean = Barcode::ean(13);
    /// assert_eq!(ean.len(), 13);
    /// ```
    pub fn ean(length: u8) -> String {
        let code = numerify(Some(&str::repeat("#", (length - 1).into())));
        format!("{}{}", code, ean::checksum(&code))
    }

    /// Get random EAN-13 barcode
    /// 
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use mock_rs::Barcode;
    ///
    /// let ean13 = Barcode::ean13();
    /// assert_eq!(ean13.len(), 13);
    /// ```
    pub fn ean13() -> String {
        Self::ean(13)
    }

    /// Get random EAN-8 barcode
    /// 
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use mock_rs::Barcode;
    ///
    /// let ean8 = Barcode::ean8();
    /// assert_eq!(ean8.len(), 8);
    /// ```
    pub fn ean8() -> String {
        Self::ean(8)
    }

    // /// Get random ISBN-10
    // pub fn isbn10() -> String {
    //     todo!()
    // }

    // /// Get random ISBN-13
    // pub fn isbn13() -> String {
    //     todo!()
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ean13() {
        assert_eq!(Barcode::ean13().len(), 13);
    }

    #[test]
    fn test_ean8() {
        assert_eq!(Barcode::ean8().len(), 8);
    }

    #[test]
    fn test_ean(){
        assert_eq!(Barcode::ean(12).len(), 12);
    }
}