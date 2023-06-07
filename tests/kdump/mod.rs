use std::{env, fs::File};

use rust_parser::RustParser;

#[test]
pub fn kdump() {
    match File::open("./tests/assets/data1.ber".to_string()) {
        Ok(file) => {
            RustParser::get_kdump_parser(&file);
            assert!(true);
        }
        Err(e) => println!("{}:Current Dir: {:#?}", e, env::current_dir()),
    }
}
