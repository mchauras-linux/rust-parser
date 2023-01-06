use std::{
    fs::File,
    io::{BufReader, Read},
};

pub(crate) struct TlvParser {
    tag_length: Vec<TagLength>,
    offset: usize,
    file: File,
}

impl TlvParser {
    pub fn new(mut file: File) -> TlvParser {
        let buf_reader = BufReader::new(&file);

        for (i, byte) in buf_reader.bytes().enumerate() {
            if let Ok(data) = byte {}
        }

        let mut tlv_parser = TlvParser {
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
