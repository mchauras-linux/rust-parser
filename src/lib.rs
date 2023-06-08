use ber_tlv::BerTlv;
use kdump::Kdump;

pub mod ber_tlv;
pub mod kdump;
pub mod traits;

/// Rust Parser core entry point
pub enum RustParser {}

impl RustParser {
    /// Get Absolute path from a relative one
    fn get_abs_path(file: &str) {
        //absolute(file);
    }

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
    pub fn get_ber_tlv_parser(file: &str) -> BerTlv {
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
    pub fn get_kdump_parser(file: &str) -> Kdump {
        Kdump::new(file)
    }
}
