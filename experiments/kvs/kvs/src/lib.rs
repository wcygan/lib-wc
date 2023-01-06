use bincode::deserialize_from;
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter, SeekFrom};
use std::path::Path;

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use crc::{Crc, CRC_31_PHILIPS};
use serde_derive::{Deserialize, Serialize};

/// The file which stores the index for the database
static DB_INDEX: &str = "kvs.index";
/// The file which stores the log of the database
static DB_FILE: &str = "kvs.db";

pub type ByteString = Vec<u8>;
pub type ByteStr = [u8];

const CRC_U32: Crc<u32> = Crc::<u32>::new(&CRC_31_PHILIPS);

#[derive(Debug, Serialize, Deserialize)]
pub struct KVPair {
    pub key: ByteString,
    pub value: ByteString,
}

/// A key-value database adapted from 'Rust in Action'
#[derive(Debug)]
pub struct KVStore {
    log: File,
    pub index: HashMap<ByteString, u64>,
}

impl KVStore {
    pub fn open() -> io::Result<Self> {
        let log = open(Path::new(DB_FILE))?;
        let index = KVStore::open_index()?;
        Ok(KVStore { log, index })
    }

    fn open_index() -> io::Result<HashMap<ByteString, u64>> {
        match deserialize_from(open(Path::new(DB_INDEX))?) {
            Ok(index) => Ok(index),
            Err(_) => {
                println!("error deserializing index, creating new index");
                Ok(HashMap::new())
            }
        }
    }

    /// Assumes that f is already at the right place in the file
    fn process_record<R: Read>(f: &mut R) -> io::Result<KVPair> {
        let saved_checksum = f.read_u32::<LittleEndian>()?;
        let key_len = f.read_u32::<LittleEndian>()?;
        let val_len = f.read_u32::<LittleEndian>()?;
        let data_len = key_len + val_len;

        let mut data = ByteString::with_capacity(data_len as usize);

        {
            f.by_ref().take(data_len as u64).read_to_end(&mut data)?;
        }

        debug_assert_eq!(data.len(), data_len as usize);

        let checksum = CRC_U32.checksum(&data);
        if checksum != saved_checksum {
            panic!(
                "data corruption encountered ({:08x} != {:08x})",
                checksum, saved_checksum
            );
        }

        let value = data.split_off(key_len as usize);
        let key = data;

        Ok(KVPair { key, value })
    }

    pub fn seek_to_end(&mut self) -> io::Result<u64> {
        self.log.seek(SeekFrom::End(0))
    }

    pub fn load(&mut self) -> io::Result<()> {
        let mut f = BufReader::new(&mut self.log);

        loop {
            let current_position = f.seek(SeekFrom::Current(0))?;
            let maybe_kv = KVStore::process_record(&mut f);
            let kv = match maybe_kv {
                Ok(kv) => kv,
                Err(err) => match err.kind() {
                    io::ErrorKind::UnexpectedEof => {
                        break;
                    }
                    _ => return Err(err),
                },
            };

            self.index.insert(kv.key, current_position);
        }

        Ok(())
    }

    pub fn get(&mut self, key: &ByteStr) -> io::Result<Option<ByteString>> {
        let position = match self.index.get(key) {
            None => return Ok(None),
            Some(position) => *position,
        };

        let kv = self.get_at(position)?;

        Ok(Some(kv.value))
    }

    pub fn get_at(&mut self, position: u64) -> io::Result<KVPair> {
        let mut f = BufReader::new(&mut self.log);
        f.seek(SeekFrom::Start(position))?;
        let kv = KVStore::process_record(&mut f)?;
        Ok(kv)
    }

    pub fn find(&mut self, target: &ByteStr) -> io::Result<Option<(u64, ByteString)>> {
        let mut f = BufReader::new(&mut self.log);

        let mut found: Option<(u64, ByteString)> = None;

        loop {
            let position = f.seek(SeekFrom::Current(0))?;

            let maybe_kv = KVStore::process_record(&mut f);
            let kv = match maybe_kv {
                Ok(kv) => kv,
                Err(err) => match err.kind() {
                    io::ErrorKind::UnexpectedEof => {
                        break;
                    }
                    _ => return Err(err),
                },
            };

            if kv.key == target {
                found = Some((position, kv.value));
            }

            // important to keep looping until the end of the file,
            // in case the key has been overwritten
        }

        Ok(found)
    }

    pub fn insert(&mut self, key: &ByteStr, value: &ByteStr) -> io::Result<()> {
        let position = self.insert_but_ignore_index(key, value)?;
        self.index.insert(key.to_vec(), position);
        Ok(())
    }

    /// Writes a variable-sized byte buffer to disk to represent the key-value pair.
    ///
    /// The format on disk is as follows:
    /// `[crc, key_len, val_len, key, value]`
    ///
    /// Where crc, key_len, and val_len all have known sizes, and key and value are variable-sized.
    /// Since the size of the first three values is known, we can use them
    ///
    /// The crc is a 32-bit checksum of the key and value, and is used to detect data corruption.
    pub fn insert_but_ignore_index(&mut self, key: &ByteStr, value: &ByteStr) -> io::Result<u64> {
        let mut f = BufWriter::new(&mut self.log);

        // Get the byte length of the key & value
        let key_len = key.len();
        let val_len = value.len();

        // Create a buffer to hold the bytes for the key & value
        let mut tmp = ByteString::with_capacity(key_len + val_len);

        // Write all of the bytes for the key
        for byte in key {
            tmp.push(*byte);
        }

        // Next, write all of the bytes for the value
        for byte in value {
            tmp.push(*byte);
        }

        let checksum = CRC_U32.checksum(&tmp);

        let next_byte = SeekFrom::End(0);
        let current_position = f.seek(SeekFrom::Current(0))?;
        f.seek(next_byte)?;

        // 1. Write the checksum (known size)
        f.write_u32::<LittleEndian>(checksum)?;
        // 2. Write the key (known size)
        f.write_u32::<LittleEndian>(key_len as u32)?;
        // 3. Write the value (known size)
        f.write_u32::<LittleEndian>(val_len as u32)?;
        // 4. write the byte buffer (variable size)
        f.write_all(&tmp)?;

        Ok(current_position)
    }

    #[inline]
    pub fn update(&mut self, key: &ByteStr, value: &ByteStr) -> io::Result<()> {
        self.insert(key, value)
    }

    #[inline]
    pub fn delete(&mut self, key: &ByteStr) -> io::Result<()> {
        self.insert(key, b"")
    }
}

impl Drop for KVStore {
    /// Serialize the index to disk when the [`KVStore`] is dropped
    fn drop(&mut self) {
        let f = match open(Path::new(DB_INDEX)) {
            Ok(f) => f,
            Err(_) => {
                panic!("error opening the index, potential data loss");
            }
        };

        match bincode::serialize(&self.index) {
            Ok(bytes) => {
                let mut writer = BufWriter::new(f);
                match writer.write_all(&bytes) {
                    Ok(_) => {}
                    Err(_) => {
                        panic!("error writing to the index, potential data loss");
                    }
                }
            }
            Err(_) => {
                panic!("error serializing index, potential data loss");
            }
        }
    }
}

fn open(f: &Path) -> io::Result<File> {
    OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(f)
}

#[cfg(test)]
mod tests {
    use super::CRC_U32;

    #[test]
    fn crc_u32_test() {
        let data = b"hello world";

        let a = CRC_U32.checksum(data);
        let b = CRC_U32.checksum(data);

        assert_eq!(a, b);
    }
}
