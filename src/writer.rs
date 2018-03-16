use std::path::Path;
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
        let p = Path::new(&path);
        let path = format!("{:?}.hack", p.file_stem().unwrap());
        File::create(path).expect("Cannot open the .hack file")
    }
}