use std::fs::File;

pub trait FileParser {
    fn get_file_path(&self) -> &str;

    fn open(&self) -> File {
        match File::open(self.get_file_path()) {
            Ok(file) => file,
            Err(e) => panic!("Error Opening file: {e}"),
        }
    }

    fn print(&self);

    fn write_to_file(&self);
}
