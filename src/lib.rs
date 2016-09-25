extern crate zip;
use zip::result::ZipError;
pub use zip::read::{ZipArchive,ZipFile};
use std::io::prelude::*;
use std::fs::{File, OpenOptions};


pub fn open<'a>(path: &str) -> Result<Vec<String>,ZipError> {
    let f: File = match OpenOptions::new().read(true).write(false).open(path) {
        Ok(x) => x,
        Err(e) => return Err(ZipError::Io(e))
    };
    let mut arc = match ZipArchive::new(f) {
        Ok(x) => x,
        Err(e) => return Err(e)
    };
    let mut v = Vec::new();
    let mut x = 0usize;
    loop {
        match arc.by_index(x) {
            Ok(mut y) => {
                let mut s = String::with_capacity(4000);
                match y.read_to_string(&mut s) {
                    Ok(_) => { },
                    Err(e) => return Err(ZipError::Io(e))
                };
                v.push(s);
                x += 1;
            },
            Err(ZipError::FileNotFound) => break,
            Err(e) => return Err(e)
        };
    }
    Ok(v)
}
