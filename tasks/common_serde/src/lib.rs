pub use common_serde_derive::{Deserialize, Serialize};
use std::io;
const I32_BYTE_BOUNDS: usize = 4; // данные о выравнивании i32
const BOOL_BYTE_BOUNDS: usize = 1; // данные о выравнивании bool

// Ошибки де/сериализации
#[derive(Debug)]
pub enum SerializeError {
    InvalidState,
}

//=========================================================================================================

pub trait Serializer {
    type Error;

    fn serialize_i32(&mut self, value: i32) -> Result<(), Self::Error>;
    fn serialize_string(&mut self, value: &str) -> Result<(), Self::Error>;
    fn serialize_bool(&mut self, value: bool) -> Result<(), Self::Error>;
}

pub trait Serialize {
    fn serialize<S: Serializer + ?Sized>(&self, serializer: &mut S) -> Result<(), S::Error>;
}

impl Serialize for i32 {
    fn serialize<S: Serializer + ?Sized>(&self, serializer: &mut S) -> Result<(), S::Error> {
        serializer.serialize_i32(*self)
    }
}

impl Serialize for String {
    fn serialize<S: Serializer + ?Sized>(&self, serializer: &mut S) -> Result<(), S::Error> {
        serializer.serialize_string(self)
    }
}

impl Serialize for bool {
    fn serialize<S: Serializer + ?Sized>(&self, serializer: &mut S) -> Result<(), S::Error> {
        serializer.serialize_bool(*self)
    }
}

impl<T> Serialize for Option<T>
where
    T: Serialize,
{
    fn serialize<S: Serializer + ?Sized>(&self, serializer: &mut S) -> Result<(), S::Error> {
        match self {
            Some(value) => {
                serializer.serialize_bool(true)?;
                value.serialize(serializer)
            }
            None => serializer.serialize_bool(false),
        }
    }
}

impl<T> Serialize for Box<T>
where
    T: Serialize + ?Sized,
{
    fn serialize<S: Serializer + ?Sized>(&self, serializer: &mut S) -> Result<(), S::Error> {
        (**self).serialize(serializer)
    }
}

impl Serializer for Vec<u8> {
    type Error = io::Error;

    fn serialize_i32(&mut self, value: i32) -> Result<(), Self::Error> {
        self.extend_from_slice(&value.to_le_bytes());
        Ok(())
    }

    fn serialize_string(&mut self, value: &str) -> Result<(), Self::Error> {
        self.extend_from_slice(&(value.len() as u32).to_le_bytes());
        self.extend_from_slice(value.as_bytes());
        Ok(())
    }

    fn serialize_bool(&mut self, value: bool) -> Result<(), Self::Error> {
        self.push(if value { 1 } else { 0 });
        Ok(())
    }
}

//=========================================================================================================

pub trait Deserializer<'a> {
    type Error;

    fn deserialize_i32(&mut self) -> Result<i32, Self::Error>;
    fn deserialize_string(&mut self) -> Result<String, Self::Error>;
    fn deserialize_bool(&mut self) -> Result<bool, Self::Error>;
}

pub trait Deserialize<'a>
where
    Self: Sized,
{
    fn deserialize<D: Deserializer<'a>>(deserializer: &mut D) -> Result<Self, D::Error>;
}

impl<'a> Deserialize<'a> for i32 {
    fn deserialize<D: Deserializer<'a>>(deserializer: &mut D) -> Result<i32, D::Error> {
        deserializer.deserialize_i32()
    }
}

impl<'a> Deserialize<'a> for String {
    fn deserialize<D: Deserializer<'a>>(deserializer: &mut D) -> Result<String, D::Error> {
        deserializer.deserialize_string()
    }
}

impl<'a> Deserialize<'a> for bool {
    fn deserialize<D: Deserializer<'a>>(deserializer: &mut D) -> Result<bool, D::Error> {
        deserializer.deserialize_bool()
    }
}

impl<'a, T> Deserialize<'a> for Option<T>
where
    T: Deserialize<'a>,
{
    fn deserialize<D: Deserializer<'a>>(deserializer: &mut D) -> Result<Self, D::Error> {
        let has_value = deserializer.deserialize_bool()?;
        match has_value {
            true => Ok(Some(T::deserialize(deserializer)?)),
            false => Ok(None),
        }
    }
}

impl<'a, T> Deserialize<'a> for Box<T>
where
    T: Deserialize<'a>,
{
    fn deserialize<D: Deserializer<'a>>(deserializer: &mut D) -> Result<Self, D::Error> {
        let value = T::deserialize(deserializer)?;
        Ok(Box::new(value))
    }
}

impl<'a> Deserializer<'a> for &'a [u8] {
    type Error = SerializeError;

    fn deserialize_i32(&mut self) -> Result<i32, Self::Error> {
        if self.len() < I32_BYTE_BOUNDS {
            return Err(SerializeError::InvalidState);
        }

        let (int_bytes, remaining) = self.split_at(I32_BYTE_BOUNDS);
        *self = remaining;

        Ok(i32::from_le_bytes(int_bytes.try_into().unwrap()))
    }

    fn deserialize_string(&mut self) -> Result<String, Self::Error> {
        let string_length = self.deserialize_i32()? as usize;

        if self.len() < string_length {
            return Err(SerializeError::InvalidState);
        }

        let (str_bytes, remaining) = self.split_at(string_length);
        *self = remaining;

        String::from_utf8(str_bytes.to_vec()).map_err(|_| SerializeError::InvalidState)
    }

    fn deserialize_bool(&mut self) -> Result<bool, Self::Error> {
        if self.is_empty() {
            return Err(SerializeError::InvalidState);
        }

        let (bool_value, remaining) = self.split_at(BOOL_BYTE_BOUNDS);
        *self = remaining;

        match bool_value[0] {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(SerializeError::InvalidState),
        }
    }
}
