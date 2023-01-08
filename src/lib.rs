#![allow(dead_code)]
use std::fs::File;

mod ber_tlv_parser;
mod traits;

pub enum RustTlvParser {
    BERTLV,
    DER,
    ASN1,
    MKV,
    JPEG,
    HEIC,
    MP4,
    AVI,
    ThreeGpp,
    ELF,
    JavaCardCap,
    RMV,
    TIFF,
    PDF,
    BMP,
    GIF,
    FLV,
}

impl RustTlvParser {
    pub fn new(parser: RustTlvParser, file: File) {
        match parser {
            RustTlvParser::BERTLV => todo!(),
            RustTlvParser::DER => todo!(),
            RustTlvParser::ASN1 => todo!(),
            RustTlvParser::MKV => todo!(),
            RustTlvParser::JPEG => todo!(),
            RustTlvParser::HEIC => todo!(),
            RustTlvParser::MP4 => todo!(),
            RustTlvParser::AVI => todo!(),
            RustTlvParser::ThreeGpp => todo!(),
            RustTlvParser::ELF => todo!(),
            RustTlvParser::JavaCardCap => todo!(),
            RustTlvParser::RMV => todo!(),
            RustTlvParser::TIFF => todo!(),
            RustTlvParser::PDF => todo!(),
            RustTlvParser::BMP => todo!(),
            RustTlvParser::GIF => todo!(),
            RustTlvParser::FLV => todo!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    #[test]
    fn it_works() {
        let _file = File::open("/home/mukesh/Work/Test/test.mkv".to_string()).unwrap();
    }
}
