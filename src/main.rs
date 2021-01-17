use async_raft::AppData;
use std::collections::hash_map::HashMap;
use std::fmt;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::SeekFrom;
use std::path::Path;

pub mod engine;
pub mod index;
pub mod memtable;
pub mod sstable;
pub mod file;

use index::Index;

fn main() {
    let mut m = HashMap::with_capacity(1);
    m.insert(0x00, String::from("lv0c0.db"));
    // let mut f = File::open("a.db").unwrap();
    // // move the cursor 42 bytes from the start of the file
    // f.seek(SeekFrom::Start(42)).unwrap();
    let index1 = Index::new(Vec::from("key1"), 10, 20, 0x01);
    let index2 = Index::new(Vec::from("key2"), 30, 10, 0x01);
    let mut indexs = Vec::with_capacity(1);
    indexs.push(index1);
    indexs.push(index2);
    flush_indexs(indexs).unwrap();
    let new_indexs = load_indexs().unwrap();
    for i in new_indexs {
        println!("{}", i);
    }
}

fn load_file_index() -> Result<HashMap<u8, String>, std::io::Error> {
    let mut m: HashMap<u8, String> = HashMap::with_capacity(10);
    let mut f = File::open("file_index.db")?;
    let mut buf = Vec::new();
    let size = f.read_to_end(&mut buf)?;
    let mut cursor = 0;
    while cursor < size {
        let size = buf[0] as usize;
        let k = buf[cursor + 1];
        let v = String::from_utf8(buf[cursor + 1..cursor + size].to_vec()).unwrap();
        m.insert(k, v);
        cursor += size
    }
    Ok(m)
}

fn flush_file_index(m: HashMap<u8, String>) -> Result<(), std::io::Error> {
    let mut buf = Vec::new();
    for (k, v) in m {
        buf.push(1u8 + v.len() as u8);
        buf.push(k);
        buf.extend(v.as_bytes());
    }

    println!("write buf:{:?}", buf);
    file::write(String::from("file_index.db"), buf.as_mut())?;
    Ok(())
}

fn load_indexs() -> Result<Vec<Index>, std::io::Error> {
    let mut f = File::open("index.db")?;
    let mut buf = Vec::new();
    let size = f.read_to_end(&mut buf)?;
    println!("read buf:{:?}, {}", buf, size);
    let indexs = index::from_buf(buf);
    Ok(indexs)
}

fn flush_indexs(indexs: Vec<Index>) -> Result<(), std::io::Error> {
    let mut buf = Vec::new();
    for idx in indexs {
        let b = idx.to_buf();
        buf.extend(b);
    }

    println!("write buf:{:?}", buf);
    file::write(String::from("index.db"), buf.as_mut())?;
    Ok(())
}

