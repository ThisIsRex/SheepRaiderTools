use std::ffi::OsStr;
use std::fs::File;
use std::io::{Error, Read, Write};

pub struct Utils {}
impl Utils {
    pub fn read_file(path: &str) -> Result<String, Error> {
        match File::open(path) {
            Ok(mut file) => {
                let mut contents = String::new();
                file.read_to_string(&mut contents).unwrap();
                Ok(contents)
            }
            Err(e) => Err(e),
        }
    }

    pub fn write_file(filename: &str, buffer: &[u8]) {
        let mut f_dest =
            File::create(&filename).expect(&format!("Cannot create {}, aborting...", filename));
        f_dest
            .write_all(&buffer)
            .expect(&format!("Cannot write {}, aborting...", filename));
    }

    pub fn get_filename_without_extension(fname: &str) -> String {
        std::path::Path::new(fname)
            .file_stem()
            .and_then(OsStr::to_str)
            .unwrap()
            .to_string()
    }
}
