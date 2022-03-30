//! Serializer and deserializer for binary data.
//!
//! Sequences and maps that encode a length prefix use a `u32` for
//! portability across platforms which limits the number of
//! items in sequences and maps to 2^32.
//!
mod deserializer;
mod error;
mod serializer;

use serde::{de::Deserialize, de::DeserializeOwned, Serialize};

use binary_rw::{BinaryReader, BinaryWriter, Endian, MemoryStream};

pub use {deserializer::Deserializer, error::Error, serializer::Serializer};

/// Result type for serialization and deserialization.
pub type Result<T> = std::result::Result<T, Error>;

/// Serialize to an owned buffer.
pub fn to_vec<T>(value: &T, endian: Endian) -> Result<Vec<u8>>
where
    T: ?Sized + Serialize,
{
    let mut stream = MemoryStream::new();
    let writer = BinaryWriter::new(&mut stream, endian);
    let mut serializer = Serializer { writer };
    value.serialize(&mut serializer)?;
    Ok(stream.into())
}

/// Deserialize from an owned buffer.
pub fn from_vec<T>(value: Vec<u8>, endian: Endian) -> Result<T>
where
    T: DeserializeOwned,
{
    let mut stream: MemoryStream = value.into();
    let reader = BinaryReader::new(&mut stream, endian);
    let mut deserializer = Deserializer { reader };
    let value: T = Deserialize::deserialize(&mut deserializer)?;
    Ok(value)
}


/// Encode into a binary buffer.
pub fn encode(encodable: &impl Encode) -> Result<Vec<u8>> {
    let mut stream = MemoryStream::new();
    let writer = BinaryWriter::new(&mut stream, Endian::Big);
    let mut serializer = Serializer { writer };
    encodable.encode(&mut serializer)?;
    Ok(stream.into())
}

/// Decode into a binary buffer.
pub fn decode<T: Decode + Default>(buffer: Vec<u8>) -> Result<T> {
    let mut stream: MemoryStream = buffer.into();
    let reader = BinaryReader::new(&mut stream, Endian::Big);
    let mut deserializer = Deserializer { reader };
    let mut decoded: T = T::default();
    decoded.decode(&mut deserializer)?;
    Ok(decoded)
}

/// Trait for encoding to serializer.
pub trait Encode {
    /// Encode self into the serializer.
    fn encode(&self, ser: &mut Serializer) -> Result<()>;
}

/// Trait for decoding from binary.
pub trait Decode {
    /// Decode from the deserializer into self.
    fn decode(&mut self, de: &mut Deserializer) -> Result<()>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;

    #[test]
    fn serde_unit() -> Result<()> {
        let val = ();
        let buffer = to_vec(&val, Default::default())?;
        let res: () = from_vec(buffer, Default::default())?;
        assert_eq!(val, res);
        Ok(())
    }

    #[test]
    fn serde_option_none() -> Result<()> {
        let val: Option<u8> = None;
        let buffer = to_vec(&val, Default::default())?;
        let res: Option<u8> = from_vec(buffer, Default::default())?;
        assert_eq!(val, res);
        Ok(())
    }

    #[test]
    fn serde_option_some() -> Result<()> {
        let val = Some(1u8);
        let buffer = to_vec(&val, Default::default())?;
        let res: Option<u8> = from_vec(buffer, Default::default())?;
        assert_eq!(val, res);
        Ok(())
    }

    #[test]
    fn serde_string() -> Result<()> {
        let val = String::from("foo");
        let buffer = to_vec(&val, Default::default())?;
        let res: String = from_vec(buffer, Default::default())?;
        assert_eq!(val, res);
        Ok(())
    }

    #[test]
    fn serde_char() -> Result<()> {
        let val = 'x';
        let buffer = to_vec(&val, Default::default())?;
        let res: char = from_vec(buffer, Default::default())?;
        assert_eq!(val, res);
        Ok(())
    }

    #[test]
    fn serde_bool_true() -> Result<()> {
        let val = true;
        let buffer = to_vec(&val, Default::default())?;
        let res: bool = from_vec(buffer, Default::default())?;
        assert_eq!(val, res);
        Ok(())
    }

    #[test]
    fn serde_bool_false() -> Result<()> {
        let val = false;
        let buffer = to_vec(&val, Default::default())?;
        let res: bool = from_vec(buffer, Default::default())?;
        assert_eq!(val, res);
        Ok(())
    }

    #[test]
    fn serde_u8() -> Result<()> {
        let val = 8u8;
        let buffer = to_vec(&val, Default::default())?;
        let res: u8 = from_vec(buffer, Default::default())?;
        assert_eq!(val, res);
        Ok(())
    }

    #[test]
    fn serde_u16() -> Result<()> {
        let val = 16u16;
        let buffer = to_vec(&val, Default::default())?;
        let res: u16 = from_vec(buffer, Default::default())?;
        assert_eq!(val, res);
        Ok(())
    }

    #[test]
    fn serde_u32() -> Result<()> {
        let val = 32u32;
        let buffer = to_vec(&val, Default::default())?;
        let res: u32 = from_vec(buffer, Default::default())?;
        assert_eq!(val, res);
        Ok(())
    }

    #[test]
    fn serde_u64() -> Result<()> {
        let val = 64u64;
        let buffer = to_vec(&val, Default::default())?;
        let res: u64 = from_vec(buffer, Default::default())?;
        assert_eq!(val, res);
        Ok(())
    }

    #[test]
    fn serde_usize() -> Result<()> {
        let val = usize::MAX;
        let buffer = to_vec(&val, Default::default())?;
        let res: usize = from_vec(buffer, Default::default())?;
        assert_eq!(val, res);
        Ok(())
    }

    #[test]
    fn serde_i8() -> Result<()> {
        let val = -8i8;
        let buffer = to_vec(&val, Default::default())?;
        let res: i8 = from_vec(buffer, Default::default())?;
        assert_eq!(val, res);
        Ok(())
    }

    #[test]
    fn serde_i16() -> Result<()> {
        let val = -16i16;
        let buffer = to_vec(&val, Default::default())?;
        let res: i16 = from_vec(buffer, Default::default())?;
        assert_eq!(val, res);
        Ok(())
    }

    #[test]
    fn serde_i32() -> Result<()> {
        let val = -32i32;
        let buffer = to_vec(&val, Default::default())?;
        let res: i32 = from_vec(buffer, Default::default())?;
        assert_eq!(val, res);
        Ok(())
    }

    #[test]
    fn serde_i64() -> Result<()> {
        let val = -64i64;
        let buffer = to_vec(&val, Default::default())?;
        let res: i64 = from_vec(buffer, Default::default())?;
        assert_eq!(val, res);
        Ok(())
    }

    #[test]
    fn serde_isize() -> Result<()> {
        let val = isize::MIN;
        let buffer = to_vec(&val, Default::default())?;
        let res: isize = from_vec(buffer, Default::default())?;
        assert_eq!(val, res);
        Ok(())
    }

    #[test]
    fn serde_vec() -> Result<()> {
        let val = vec![1u8, 2u8, 3u8];
        let buffer = to_vec(&val, Default::default())?;
        let res: Vec<u8> = from_vec(buffer, Default::default())?;
        assert_eq!(val, res);
        Ok(())
    }

    #[test]
    fn serde_tuple() -> Result<()> {
        let val = (1u8, String::from("foo"));
        let buffer = to_vec(&val, Default::default())?;
        let res: (u8, String) = from_vec(buffer, Default::default())?;
        assert_eq!(val, res);
        Ok(())
    }

    #[test]
    fn serde_map() -> Result<()> {
        let mut val = HashMap::new();
        val.insert("foo".to_string(), 1u8);
        val.insert("bar".to_string(), 2u8);
        let buffer = to_vec(&val, Default::default())?;
        let res: HashMap<String, u8> = from_vec(buffer, Default::default())?;
        assert_eq!(val, res);
        Ok(())
    }

    #[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
    struct SimpleStruct {
        x: u32,
        y: u32,
    }

    #[test]
    fn serde_struct() -> Result<()> {
        let val = SimpleStruct { x: 1, y: 2 };
        let buffer = to_vec(&val, Default::default())?;
        let res: SimpleStruct = from_vec(buffer, Default::default())?;
        assert_eq!(val, res);
        Ok(())
    }

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    enum E {
        Unit,
        NewType(u32),
        Tuple(u32, u32),
        Struct { a: u32 },
    }

    #[test]
    fn serde_enum_unit() -> Result<()> {
        let val = E::Unit;
        let buffer = to_vec(&val, Default::default())?;
        let res: E = from_vec(buffer, Default::default())?;
        assert_eq!(val, res);
        Ok(())
    }

    #[test]
    fn serde_enum_newtype() -> Result<()> {
        let val = E::NewType(1);
        let buffer = to_vec(&val, Default::default())?;
        let res: E = from_vec(buffer, Default::default())?;
        assert_eq!(val, res);
        Ok(())
    }

    #[test]
    fn serde_enum_tuple() -> Result<()> {
        let val = E::Tuple(1, 2);
        let buffer = to_vec(&val, Default::default())?;
        let res: E = from_vec(buffer, Default::default())?;
        assert_eq!(val, res);
        Ok(())
    }

    #[test]
    fn serde_enum_struct() -> Result<()> {
        let val = E::Struct { a: 1 };
        let buffer = to_vec(&val, Default::default())?;
        let res: E = from_vec(buffer, Default::default())?;
        assert_eq!(val, res);
        Ok(())
    }

    #[derive(Debug, Eq, PartialEq)]
    struct TodoList {
        magic: [u8; 4],
        todos: Vec<Todo>,
    }

    impl Default for TodoList {
        fn default() -> Self {
            Self {
                magic: [84, 79, 68, 79],
                todos: vec![],
            }
        }
    }

    impl Encode for TodoList {
        fn encode(&self, ser: &mut Serializer) -> super::Result<()> {
            ser.writer.write_bytes(self.magic)?;
            self.todos.serialize(ser)?;
            Ok(())
        }
    }

    impl Decode for TodoList {
        fn decode(&mut self, de: &mut Deserializer) -> super::Result<()> {
            self.magic = de.reader.read_bytes(4)?.as_slice().try_into()
                .map_err(|_| {
                    Error::Message(
                        "cannot convert magic bytes to slice".to_string())
                })?;
            self.todos = Deserialize::deserialize(de)?;
            Ok(())
        }
    }

    #[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
    struct Todo {
        name: String,
        note: String,
    }

    #[test]
    fn serde_mixed() -> Result<()> {
        let todos = vec![
            Todo {
                name: String::from("foo"),
                note: String::from("bar")
            }
        ];

        let list = TodoList {
            magic: [84, 79, 68, 79],
            todos,
        };

        let buffer = encode(&list)?;
        let decoded: TodoList = decode(buffer)?;
        assert_eq!(list, decoded);
        Ok(())
    }
}
