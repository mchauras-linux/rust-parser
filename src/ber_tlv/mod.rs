pub(crate) mod tag_length;
pub mod tlv_parser_error;

use crate::traits::RustParser;
use std::{
    fs::File,
    io::{BufReader, Read},
};

use self::tag_length::TagLength;

#[derive(Debug)]
pub struct BerTlv<'a> {
    tag_length: Vec<TagLength>,
    offset: usize,
    file: &'a File,
}

impl<'a> RustParser for BerTlv<'a> {
    fn write_to_file() {
        todo!()
    }

    fn print(&self) {
        let buf_reader = BufReader::new(self.file);

        for (_i, byte) in buf_reader.bytes().enumerate() {
            if let Ok(data) = byte {
                print!("{:02X}", data);
            }
        }
    }
}

impl<'a> BerTlv<'a> {
    pub(crate) fn new(file: &File) -> BerTlv {
        let tlv_parser = BerTlv {
            offset: 0,
            file,
            tag_length: vec![TagLength {
                val_offset: 0,
                tag: vec![1, 2, 3, 4],
                len: 3,
            }],
        };

        tlv_parser.print();
        return tlv_parser;
    }
}
