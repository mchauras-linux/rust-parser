use rust_parser::RustParser;
use std::{env, fs::File};

#[test]
pub fn ber_tlv() {
    match File::open("./tests/assets/data1.ber".to_string()) {
        Ok(file) => {
            RustParser::get_ber_tlv_parser(file);
            assert!(true);
        }
        Err(e) => println!("{}:Current Dir: {:#?}", e, env::current_dir()),
    }
}
