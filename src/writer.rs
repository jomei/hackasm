use std::path::PathBuf;
use std::fs::File;
use std::io::Write;

pub struct Writer {
    file: File
}

impl Writer {
    pub fn new(file_path: String) -> Self {
        let file = Writer::create_file(file_path);
        Writer {file}
    }

    pub fn call(&mut self, line: String) {
        writeln!(&mut self.file, "{}", line);
    }

    fn create_file(path: String) -> File {
        let mut file_name = PathBuf::from(&path);
        file_name.set_extension("hack");

        File::create(file_name).expect("Cannot open the .hack file")
    }
}