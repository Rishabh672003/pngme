#![allow(unused_variables, unused)]
use crate::chunk_type::ChunkType;
use std::convert::TryFrom;

#[derive(Debug)]
pub struct Chunk {
    length: u32,
    chunk_type: ChunkType,
    chunk_data: Vec<u8>,
    crc: u32,
}

#[derive(Debug)]
pub enum InvalidChunk {
    Header,
    Length,
    Type,
    Data,
    Crc,
}

// impl

impl std::fmt::Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ", self.length)?;
        write!(f, "{} ", self.chunk_type)?;
        write!(
            f,
            "{} ",
            std::str::from_utf8(&self.chunk_data).unwrap(),
        )?;
        write!(f, "{} ", self.crc)
    }
}

impl Chunk {
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Self {
        let length = data.len() as u32;
        let mut whole = chunk_type.bytes().to_vec();
        whole.extend_from_slice(&data);
        const X25: crc::Crc<u32> = crc::Crc::<u32>::new(&crc::CRC_32_ISO_HDLC);
        let val = X25.checksum(&whole);
        Self {
            length,
            chunk_type,
            chunk_data: data,
            crc: val,
        }
    }
    pub fn length(&self) -> u32 {
        self.length
    }
    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }
    pub fn data(&self) -> &[u8] {
        &self.chunk_data
    }
    pub fn data_as_string(&self) -> Result<String, InvalidChunk> {
        match String::from_utf8(self.chunk_data.clone()) {
            Ok(val) => Ok(val),
            Err(_) => { 
                eprintln!("{:?}", self.data());
                Err(InvalidChunk::Data)?
            },
        }
    }
    pub fn crc(&self) -> u32 {
        self.crc
    }
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut ans = vec![];
        ans.extend_from_slice(&self.length.to_be_bytes());
        ans.extend_from_slice(&self.chunk_type.bytes());
        ans.extend_from_slice(&self.chunk_data);
        ans.extend_from_slice(&self.crc.to_be_bytes());
        ans
    }
}

impl std::fmt::Display for InvalidChunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InvalidChunk::Length => write!(f, "Invalid Length"),
            InvalidChunk::Header => write!(f, "Invalid Header"),
            InvalidChunk::Type => write!(f, "Invalid Type"),
            InvalidChunk::Data => write!(f, "Invalid Data"),
            InvalidChunk::Crc => write!(f, "Invalid Crc"),
        }
    }
}

impl std::error::Error for InvalidChunk {}

impl TryFrom<&[u8]> for Chunk {
    type Error = InvalidChunk;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let length = u32::from_be_bytes([value[0], value[1], value[2], value[3]]);
        let chunk_type = ChunkType::try_from([value[4], value[5], value[6], value[7]]).unwrap();
        let len = value.len();
        if (length as usize) != len - 12 {
            Err(InvalidChunk::Length)?
        }
        let chunk_data = value[8..(8 + length as usize)].to_vec();

        let crc = u32::from_be_bytes([
            value[len - 4],
            value[len - 3],
            value[len - 2],
            value[len - 1],
        ]);
        let chunk = Self {
            length,
            chunk_type,
            chunk_data,
            crc,
        };

        const X25: crc::Crc<u32> = crc::Crc::<u32>::new(&crc::CRC_32_ISO_HDLC);
        let val = X25.checksum(&value[4..len - 4]);
        if val != crc {
            Err(InvalidChunk::Crc)?
        }
        Ok(chunk)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!"
            .as_bytes()
            .to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();

        let _chunk_string = format!("{}", chunk);
    }
}
