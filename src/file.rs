use std::io;
use std::{fs::File, io::Read};
use std::path::Path;
use std::io::prelude::*;
use std::io::SeekFrom;

pub fn file_append(file: &mut File, value: &[u8]) -> Result<(), std::io::Error> {
    match file.write(value) {
        Err(why) => {
            println!("write file error {}", why);
            Err(why)
        }
        Ok(_) => file.sync_all(),
    }
}

//追加写入
pub fn write_append(filename: String, value: &[u8]) -> Result<(), std::io::Error> {
    let path = Path::new(&filename);
    match std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(path)
    {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => Err(why),
        Ok(ref mut file) => file_append(file, value),
    }
}

//直接写入
pub fn write(filename: String, value: &[u8]) -> Result<(), std::io::Error> {
    let path = Path::new(&filename);
    match std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(path)
    {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => Err(why),
        Ok(ref mut file) => file_write(file, value, 0),
    }
}

pub fn file_write(file: &mut File, value: &[u8], offset: u64) -> Result<(), std::io::Error> {
    if offset != 0 {
        file.seek(std::io::SeekFrom::Start(offset))?;
    }

    match file.write(value) {
        Err(why) => {
            println!("write file error {}", why);
            Err(why)
        }
        Ok(_) => file.sync_all(),
    }
}
