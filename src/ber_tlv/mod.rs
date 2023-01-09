pub(crate) mod tag_length;
pub mod tlv_parser_error;

use crate::traits::RustParser;
use std::{
    fs::File,
    io::{BufReader, Read},
};

use self::tag_length::TagLength;

pub struct BerTlv {
    tag_length: Vec<TagLength>,
    offset: usize,
    file: File,
}

impl RustParser for BerTlv {
    fn write_to_file() {
        todo!()
    }
}

impl BerTlv {
    pub(crate) fn new(mut file: File) -> BerTlv {
        let buf_reader = BufReader::new(&file);

        for (i, byte) in buf_reader.bytes().enumerate() {
            if let Ok(data) = byte {
                print!("{:02X}", data);
            }
        }

        let mut tlv_parser = BerTlv {
            offset: 0,
            file: file,
            tag_length: vec![TagLength {
                val_offset: 0,
                tag: vec![1, 2, 3, 4],
                len: 3,
            }],
        };
        return tlv_parser;
    }
}
