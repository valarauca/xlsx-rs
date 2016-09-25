extern crate zip;
use zip::result::ZipError;
pub use zip::read::{ZipArchive,ZipFile};
use std::io::prelude::*;
use std::fs::{File, OpenOptions};

pub struct WorkSheet {
    name: String,
    data: String
}
impl WorkSheet {
    pub fn new(mut a: ZipFile) -> Result<WorkSheet,ZipError> {
        let mut x = WorkSheet {
            name: a.name().to_string(),
            data: unsafe{ ::std::mem::uninitialized()}
        };
        let mut s = String::with_capacity(4000);
        match a.read_to_string(&mut s) {
            Ok(_) => { },
            Err(e) => return Err(ZipError::Io(e))
        };
        x.data = s;
        Ok(x)
    }
}


pub fn open(path: &str) -> Result<Vec<WorkSheet>,ZipError> {
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
            Ok(y) => match WorkSheet::new(y) {
                Err(e) => return Err(e),
                Ok(z) => {
                    v.push(z);
                    x += 1;
                }
            },
            Err(ZipError::FileNotFound) => break,
            Err(e) => return Err(e)
        };
    }
    Ok(v)
}
