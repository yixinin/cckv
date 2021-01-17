use skiplist::SkipList;
use std::collections::HashMap;

use std::io::Error;

use crate::{memtable, sstable, Index}; 
pub struct Engine {
    db_name: String,
    file_index: HashMap<u8, String>,
    indexs: SkipList<Index>,
    wait_deletes: Vec<Index>,

    map: HashMap<Vec<u8>, Vec<u8>>,
    wait_dumps: Vec<u8>,
}

impl Engine {
    fn new(db: String) -> Self {
        Engine {
            db_name: db,
            file_index: HashMap::new(),
            indexs: SkipList::new(),
            wait_deletes: Vec::new(),
            map: HashMap::new(),
            wait_dumps: Vec::new(),
        }
    }

    // load stored keys index from disk
    fn load(&mut self) -> Result<(), Error> {
        Ok(())
    }
    fn get(self, key: Vec<u8>) -> Result<Vec<u8>, Error> {
        let val = Vec::new();

        Ok(val)
    }

    fn put(&mut self, key: Vec<u8>, val: Vec<u8>) -> Result<(), Error> {
        self.map.insert(key, val);
        Ok(())
    }
    fn delete(&mut self, key: Vec<u8>) -> Result<(), Error> {
        Ok(())
    }

    fn scan(
        self,
        start_key: Vec<u8>,
        end_key: Vec<u8>,
        limit: usize,
    ) -> Result<Vec<(Vec<u8>, Vec<u8>)>, Error> {
        let ret = Vec::new();
        Ok(ret)
    }

    fn batch_get(self, keys: Vec<Vec<u8>>) -> Result<Vec<(Vec<u8>, Vec<u8>)>, Error> {
        let mut ret = Vec::new();
        Ok(ret)
    }
    fn batch_put(&mut self, keys: Vec<Vec<u8>>, vals: Vec<Vec<u8>>) -> Result<Vec<usize>, Error> {
        let mut ret = Vec::new();
        Ok(ret)
    }
    fn batch_delete(&mut self, keys: Vec<Vec<u8>>) -> Result<Vec<usize>, Error> {
        let mut ret = Vec::new();
        Ok(ret)
    }

    fn flush(self) -> Result<(), Error> {
        Ok(())
    }

    fn merge(self) -> Result<(),Error>{
        Ok(())
    }
}
