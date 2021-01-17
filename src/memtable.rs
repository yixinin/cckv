use std::io::Error;
use std::{convert::TryInto, io};
use std::{fs::File, io::Read};

use index::Index;

use crate::{file, index};
#[derive(Clone, Debug)]
pub struct Kv {
    Key: Vec<u8>,
    Val: Vec<u8>,
}

impl Kv {
    fn size(&self) -> usize {
        return self.Key.len() + self.Val.len();
    }
    fn key_size(&self) -> u32 {
        return self.Key.len() as u32;
    }
    fn val_size(&self) -> u32 {
        return self.Val.len() as u32;
    }
}
pub struct KvStore {}

impl KvStore {
    fn merge(self, kvs1: Vec<Kv>, kvs2: Vec<Kv>) -> Vec<Kv> {
        let mut kvs = Vec::with_capacity(kvs1.len() + kvs2.len());
        let mut cursor1 = 0;
        let mut cursor2 = 0;
        while cursor1 < kvs1.len() || cursor2 < kvs2.len() {
            if kvs1[cursor1].Key <= kvs2[cursor2].Key {
                kvs.push(kvs1[cursor1].clone());
                cursor1 += 1
            } else {
                kvs.push(kvs2[cursor2].clone());
                cursor2 += 1
            }
        }
        return kvs;
    }

    fn load(self, f: &mut File) -> Result<Vec<Kv>, Error> {
        let mut kvs = Vec::new();

        let mut buf = Vec::new();
        let size = f.read_to_end(&mut buf)?;
        let mut cursor = 0;

        while cursor < size {
            let mut key_slice = [0u8; 4];
            let mut val_slice = [0u8; 4];
            let deleted = buf[cursor] == 1;
            for i in 0..8 {
                key_slice[i] = buf[cursor + i];
                val_slice[i] = buf[cursor + 4 + i];
            }
            let key_size = unsafe { std::mem::transmute::<[u8; 4], u32>(key_slice) } as usize;
            let val_size = unsafe { std::mem::transmute::<[u8; 4], u32>(val_slice) } as usize;
            cursor += key_size + val_size + 1 + 8;
            if deleted {
                continue;
            }
            let kv = Kv {
                Key: buf[cursor + 8..cursor + 8 + key_size].to_vec(),
                Val: buf[cursor + 8 + key_size..cursor + 8 + key_size + val_size].to_vec(),
            };
            kvs.push(kv);
        }
        Ok(kvs)
    }

    fn flush(self, f: &mut File, kvs: Vec<Kv>) -> Result<Vec<Index>, Error> {
        let mut indexs = Vec::with_capacity(kvs.len());
        let mut buf = Vec::with_capacity(get_size(&kvs));
        for kv in kvs {
            let size = kv.size() + 1 + 8;
            let index = Index {
                key: kv.Key.clone(),
                offset: buf.len(),
                size: size,
                file_index: 0,
            };
            indexs.push(index);
            buf.push(0);
            buf.extend(unsafe { std::mem::transmute::<u32, [u8; 4]>(kv.key_size()).to_vec() });
            buf.extend(unsafe { std::mem::transmute::<u32, [u8; 4]>(kv.val_size()).to_vec() });
            buf.extend(kv.Key);
            buf.extend(kv.Val);
        }
        file::file_write(f, &buf, 0)?;
        Ok(indexs)
    }
}
fn get_size(kvs: &Vec<Kv>) -> usize {
    let mut size = 0;
    for kv in kvs {
        size += 8 * 2 + kv.size();
    }
    return size;
}
