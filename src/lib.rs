use std::fs::File;

use ber_tlv::BerTlv;
use kdump::Kdump;

mod ber_tlv;
mod kdump;
mod traits;

/// Rust Parser core entry point
pub enum RustParser {}

impl RustParser {
    /// Get BER-TLV Parser object
    /// # Example
    /// ```
    /// use rust_parser::RustParser;
    /// use std::fs::File;
    /// match File::open("data1.ber".to_string()) {
    ///     Ok(file) => {
    ///         RustParser::get_ber_tlv_parser(&file);
    ///         assert!(true);
    ///     }
    ///     Err(e) => println!("{}", e),
    /// }
    ///
    /// ```
    pub fn get_ber_tlv_parser(file: &File) -> BerTlv {
        BerTlv::new(file)
    }

    /// Get KDUMP Parser object
    /// # Example
    /// ```
    /// use rust_parser::RustParser;
    /// use std::fs::File;
    /// match File::open("vmcore".to_string()) {
    ///     Ok(file) => {
    ///         RustParser::get_kdump_parser(&file);
    ///         assert!(true);
    ///     }
    ///     Err(e) => println!("{}", e),
    /// }
    ///
    /// ```
    pub fn get_kdump_parser(file: &File) -> Kdump {
        Kdump::new(file)
    }
}
