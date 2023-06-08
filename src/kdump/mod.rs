use std::io::{BufReader, Read};

use crate::traits::FileParser;

#[derive(Debug)]
pub struct Kdump<'a> {
    file: &'a str,
}

impl<'c> FileParser for Kdump<'c> {
    fn get_file_path(&self) -> &str {
        self.file
    }

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
}

impl<'b> Kdump<'b> {
    pub(crate) fn new(file: &'b str) -> Self {
        Self { file }
    }
}
