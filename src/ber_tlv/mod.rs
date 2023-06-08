pub(crate) mod tag_length;
pub mod tlv_parser_error;

use crate::traits::FileParser;
use std::io::{BufReader, Read};

use self::tag_length::TagLength;

#[derive(Debug)]
pub struct BerTlv<'a> {
    _tag_length: Vec<TagLength>,
    _offset: usize,
    file: &'a str,
}

impl<'a> FileParser for BerTlv<'a> {
    fn print(&self) {
        let file = self.open();
        let buf_reader = BufReader::new(file);

        for (_i, byte) in buf_reader.bytes().enumerate() {
            if let Ok(data) = byte {
                print!("{:02X}", data);
            }
        }
    }

    fn write_to_file(&self) {
        todo!()
    }

    fn get_file_path(&self) -> &str {
        self.file
    }
}

impl<'a> BerTlv<'a> {
    pub(crate) fn new(file: &'a str) -> BerTlv {
        let tlv_parser = BerTlv {
            _offset: 0,
            file,
            _tag_length: vec![TagLength {
                _val_offset: 0,
                _tag: vec![1, 2, 3, 4],
                _len: 3,
            }],
        };

        tlv_parser.print();
        return tlv_parser;
    }
}
