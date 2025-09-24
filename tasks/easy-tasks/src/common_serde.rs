#[derive(Debug, PartialEq)]
pub struct MyStruct {
    pub value_i32: i32,
    pub value_string: String,
}

//=========================================================================================

// Ошибки сериализации
#[derive(Debug)]
pub enum SerializeError {
    InvalidState,
}

//=========================================================================================

// Трейт для обхода полей структуры
pub trait Serializer {
    type Output;
    type Error;

    fn serialize_i32(&mut self, value: i32) -> Result<(), Self::Error>;
    fn serialize_string(&mut self, value: &str) -> Result<(), Self::Error>;
    fn finish(self) -> Self::Output;
}

impl Serializer for Vec<u8> {
    type Output = Vec<u8>;
    type Error = SerializeError;

    fn serialize_i32(&mut self, value: i32) -> Result<(), Self::Error> {
        let bytes = value.to_le_bytes();
        self.extend_from_slice(&bytes);
        Ok(())
    }

    fn serialize_string(&mut self, value: &str) -> Result<(), Self::Error> {
        let length = value.len() as u32;
        self.extend_from_slice(&length.to_le_bytes());
        self.extend_from_slice(&value.as_bytes());
        Ok(())
    }

    fn finish(self) -> Self::Output {
        self
    }
}

// Трейт для структур подлежащих сериализации.
pub trait Serialize {
    fn serialize<S: Serializer>(&self, serializer: &mut S) -> Result<(), S::Error>;
}

impl Serialize for MyStruct {
    fn serialize<S: Serializer>(&self, serializer: &mut S) -> Result<(), S::Error> {
        serializer.serialize_i32(self.value_i32)?;
        serializer.serialize_string(&self.value_string)?;
        Ok(())
    }
}

//=========================================================================================
const I32_BYTE_BOUNDS: usize = 4; // данные о выравнивании i32

pub trait Deserializer<'a> {
    type Error;

    fn deserialize_i32(&mut self) -> Result<i32, Self::Error>;
    fn deserialize_string(&mut self) -> Result<String, Self::Error>;
}

impl<'a> Deserializer<'a> for &'a [u8] {
    type Error = SerializeError;

    fn deserialize_i32(&mut self) -> Result<i32, Self::Error> {
        if self.len() < I32_BYTE_BOUNDS {
            return Err(SerializeError::InvalidState);
        }

        let (int_bytes, rest) = self.split_at(4);
        *self = rest;

        Ok(i32::from_le_bytes(int_bytes.try_into().unwrap()))
    }

    fn deserialize_string(&mut self) -> Result<String, Self::Error> {
        let string_length = self.deserialize_i32()? as usize;

        if self.len() < string_length {
            return Err(SerializeError::InvalidState);
        }

        let (str_bytes, rest) = self.split_at(string_length);
        *self = rest;

        String::from_utf8(str_bytes.to_vec()).map_err(|_| SerializeError::InvalidState)
    }
}

// Трейт для структур подлежащих десериализации.
pub trait Deserialize<'a>: Sized {
    fn deserialize<D: Deserializer<'a>>(deserializer: &mut D) -> Result<Self, D::Error>;
}

impl<'a> Deserialize<'a> for MyStruct {
    fn deserialize<D: Deserializer<'a>>(deserializer: &mut D) -> Result<Self, D::Error> {
        let value_i32 = deserializer.deserialize_i32()?;
        let value_string = deserializer.deserialize_string()?;

        Ok(MyStruct {
            value_i32,
            value_string,
        })
    }
}
