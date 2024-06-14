use anyhow::Result;
use std::io::Read;

pub trait Serialize {
    fn serialize(&self, writer: &mut impl std::io::Write) -> Result<()>;
}

pub trait Deserialize: Sized {
    fn deserialize(reader: &mut impl std::io::Read) -> Result<Self>;
}

impl Serialize for &str {
    fn serialize(&self, writer: &mut impl std::io::Write) -> Result<()> {
        Ok(rmp::encode::write_str(writer, self)?)
    }
}

impl Deserialize for String {
    fn deserialize(reader: &mut impl std::io::Read) -> Result<Self> {
        let len = rmp::decode::read_str_len(reader)?;
        read_to_string(reader, len as usize)
    }
}

pub fn read_to_string(reader: &mut impl std::io::Read, len: usize) -> Result<String> {
    let mut value = String::new();
    reader.take(len as u64).read_to_string(&mut value)?;
    Ok(value)
}
