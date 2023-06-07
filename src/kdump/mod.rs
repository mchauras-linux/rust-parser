use std::fs::File;

use crate::traits::RustParser;

#[derive(Debug)]
pub struct Kdump<'a> {
    file: &'a File,
}

impl<'a> RustParser for Kdump<'a> {
    fn write_to_file() {
        todo!()
    }

    fn print(&self) {
        todo!()
    }
}

impl<'a> Kdump<'a> {
    pub(crate) fn new(file: &'a File) -> Self {
        Self { file }
    }
}
