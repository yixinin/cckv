use std::fmt;

#[derive(Debug)]
pub struct Index {
   pub key: Vec<u8>,   // key
   pub file_index: u8, // 所在的文件编号
   pub offset: usize,  // 偏移量
   pub size: usize,    // 长度
}

type IndexSlice = Vec<Index>;

fn to_buf(s: &mut IndexSlice) -> Vec<u8> {
    let mut buf = Vec::with_capacity(indexs_size(s));
    for i in s {
        buf.extend(i.to_buf());
    }
    return buf;
}

fn indexs_size(s: &IndexSlice) -> usize {
    let mut size = 0;
    for i in s {
        size += i.size();
    }
    return size as usize;
}

pub fn from_buf(buf: Vec<u8>) -> IndexSlice {
    let length = buf.len();
    let mut indexs = Vec::with_capacity(length / 32);

    let size = buf[0] as usize;
    let mut cursor = 0;
    while cursor < length {
        let index = Index::from_buf(&buf[cursor..cursor + size + 1]);
        indexs.push(index);
        cursor += size + 1
    }
    return indexs;
}

impl fmt::Display for Index {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "key:{}, offset:{}, size:{}, filename:{}",
            String::from_utf8(self.key.clone()).unwrap(),
            self.offset,
            self.size,
            self.file_index,
        )
    }
}

impl Index {
    pub fn new(key: Vec<u8>, offset: usize, size: usize, file_index: u8) -> Self {
        Index {
            key,
            offset,
            size,
            file_index,
        }
    }

    fn default() -> Self {
        Index {
            key: Vec::new(),
            offset: 0,
            size: 0,
            file_index: 0,
        }
    }

    fn min_cap(&self) -> usize {
        return 17;
    }

    fn size(&self) -> u8 {
        if self.key.len() > 256 - self.min_cap() {
            panic!("key len max than 246");
        }
        return (self.min_cap() + self.key.len()) as u8;
    }

    pub fn from_buf(buf: &[u8]) -> Self {
        println!("{:?}", buf);
        let mut index = Index::default();

        let size = buf[0] as usize;

        index.key = buf[1..1 + (size - index.min_cap())].to_vec();

        let key_offset = index.key.len() + 1;
        // read offset and size
        {
            let mut offset_buf = [0u8; 8];
            let mut size_buf = [0u8; 8];
            for i in 0..8 {
                offset_buf[i] = buf[key_offset + i];
                size_buf[i] = buf[key_offset + 8 + i];
            }
            index.offset = unsafe { std::mem::transmute::<[u8; 8], usize>(offset_buf) };
            index.size = unsafe { std::mem::transmute::<[u8; 8], usize>(size_buf) };
        }

        index.file_index = buf[size];
        return index;
    }

    // [head:vals]
    pub fn to_buf(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(self.size() as usize + 1);
        buf.push(self.size()); // 头部 标识长度

        unsafe {
            buf.extend(self.key.clone());

            let offset_buf = std::mem::transmute::<usize, [u8; 8]>(self.offset);
            buf.extend(offset_buf.to_vec());

            let size_buf = std::mem::transmute::<usize, [u8; 8]>(self.size);
            buf.extend(size_buf.to_vec());

            buf.push(self.file_index);
        }
        return buf;
    }
}
