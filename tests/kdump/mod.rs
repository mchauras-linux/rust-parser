use rust_parser::{kdump::Kdump, traits::FileParser, RustParser};

#[test]
pub fn kdump() {
    let kdump: Kdump = RustParser::get_kdump_parser("./tests/assets/ber-tlv/data1.ber");
    kdump.print();
    assert!(true);
}
