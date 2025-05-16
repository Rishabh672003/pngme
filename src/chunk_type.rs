#![allow(unused)]

use std::fmt::Display;

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct ChunkType {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
}

#[derive(Debug)]
pub enum ChunkTypeError {
    InvalidChunk,
}

impl std::fmt::Display for ChunkTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChunkTypeError::InvalidChunk => write!(f, "Invalid Chunk")?,
        }
        Ok(())
    }
}
impl std::error::Error for ChunkTypeError {}

fn check(val: u8) -> Result<u8, ChunkTypeError> {
    if (65..91).contains(&val) || (97..123).contains(&val) {
        Ok(val)
    } else {
        Err(ChunkTypeError::InvalidChunk)?
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = ChunkTypeError;
    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        Ok({
            Self {
                a: check(value[0])?,
                b: check(value[1])?,
                c: check(value[2])?,
                d: check(value[3])?,
            }
        })
    }
}
impl std::str::FromStr for ChunkType {
    type Err = ChunkTypeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let byte = s.as_bytes();
        if byte.len() != 4 {
            return Err(ChunkTypeError::InvalidChunk);
        }
        Ok(Self {
            a: check(byte[0])?,
            b: check(byte[1])?,
            c: check(byte[2])?,
            d: check(byte[3])?,
        })
    }
}

impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        [self.a, self.b, self.c, self.d]
    }
    fn is_valid(&self) -> bool {
        b'A' <= self.c && b'Z' >= self.c
    }
    fn is_critical(&self) -> bool {
        self.a & (1 << 5) == 0
    }
    fn is_public(&self) -> bool {
        self.b & (1 << 5) == 0
    }
    fn is_reserved_bit_valid(&self) -> bool {
        self.c & (1 << 5) == 0
    }
    fn is_safe_to_copy(&self) -> bool {
        self.d & (1 << 5) != 0
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = String::from_utf8(vec![self.a, self.b, self.c, self.d]).unwrap();
        write!(f, "{}", str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}
