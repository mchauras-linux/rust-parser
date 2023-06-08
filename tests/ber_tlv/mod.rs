use rust_parser::{ber_tlv::BerTlv, traits::FileParser, RustParser};

#[test]
pub fn ber_tlv() {
    let parser: BerTlv = RustParser::get_ber_tlv_parser("./tests/assets/data1.ber");
    parser.print();
}
