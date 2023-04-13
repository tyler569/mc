use anyhow::Result;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::io::{Read, Write};

const SEGMENT_BITS: u32 = 0x7f;
const CONTINUE_BIT: u8 = 0x80;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct VarInt(pub i32);

#[derive(Copy, Clone, Debug)]
pub enum VarIntError {
    NotEnoughBytes,
    TooManyBytes,
}

impl Display for VarIntError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for VarIntError {}

fn read_one(reader: &mut impl Read) -> Option<u8> {
    let v = &mut [0u8];
    reader.read(v).ok()?;
    Some(v[0])
}

impl VarInt {
    pub fn write(self, writer: &mut impl Write) -> Result<()> {
        let mut buffer = [0u8; 6];
        let mut index = 0;
        let mut value = self.0 as u32;
        loop {
            if value & !SEGMENT_BITS == 0 {
                buffer[index] = value as u8;
                index += 1;
                writer.write(&buffer[0..index])?;
                return Ok(());
            }

            buffer[index] = value as u8 | CONTINUE_BIT;
            index += 1;
            value >>= 7;
        }
    }

    pub fn read(reader: &mut impl Read) -> Result<Self> {
        let mut value: u32 = 0;
        let mut position = 0;
        let mut current_byte = 0;
        loop {
            current_byte = read_one(reader).ok_or(VarIntError::NotEnoughBytes)?;
            value |= (current_byte as u32 & SEGMENT_BITS) << position;

            if current_byte & CONTINUE_BIT == 0 {
                break;
            }
            position += 7;
            if position > 32 {
                Err(VarIntError::TooManyBytes)?;
            }
        }
        Ok(Self(value as i32))
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct VarLong(pub i64);

impl VarLong {
    pub fn write(self, writer: &mut dyn Write) -> Result<()> {
        let mut buffer = [0u8; 10];
        let mut index = 0;
        let mut value = self.0 as u64;
        loop {
            if value & !(SEGMENT_BITS as u64) == 0 {
                buffer[index] = value as u8;
                index += 1;
                writer.write(&buffer[0..index])?;
                return Ok(());
            }

            buffer[index] = value as u8 | CONTINUE_BIT;
            index += 1;
            value >>= 7;
        }
    }

    pub fn read(reader: &mut impl Read) -> Result<Self> {
        let mut value: u64 = 0;
        let mut position = 0;
        let mut current_byte = 0;
        loop {
            current_byte = read_one(reader).ok_or(VarIntError::NotEnoughBytes)?;
            value |= (current_byte as u64 & SEGMENT_BITS as u64) << position;

            if current_byte & CONTINUE_BIT == 0 {
                break;
            }
            position += 7;
            if position > 32 {
                Err(VarIntError::TooManyBytes)?;
            }
        }
        Ok(Self(value as i64))
    }
}

#[cfg(test)]
mod tests {
    const VARINT_CONVERSIONS: &[(i32, &[u8])] = &[
        (0, &[0]),
        (1, &[1]),
        (2, &[2]),
        (127, &[127]),
        (128, &[128, 1]),
        (255, &[255, 1]),
        (25565, &[221, 199, 1]),
        (2097151, &[255, 255, 127]),
        (2147483647, &[255, 255, 255, 255, 7]),
        (-1, &[255, 255, 255, 255, 15]),
        (-2147483648, &[128, 128, 128, 128, 8]),
    ];

    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_varint_encode() {
        let mut vec = vec![];
        for &(number, bytes) in VARINT_CONVERSIONS {
            VarInt(number).write(&mut vec);
            assert_eq!(vec, bytes);
            vec.clear();
        }
    }

    #[test]
    fn test_varint_decode() {
        for &(number, bytes) in VARINT_CONVERSIONS {
            let mut vec = Vec::from(bytes);
            let mut reader = Cursor::new(vec);
            let varint_value = VarInt::read(&mut reader).unwrap();
            assert_eq!(number, varint_value.0);
        }
    }

    const VARLONG_CONVERSIONS: &[(i64, &[u8])] = &[
        (0, &[0]),
        (1, &[1]),
        (2, &[2]),
        (127, &[127]),
        (128, &[128, 1]),
        (255, &[255, 1]),
        (25565, &[221, 199, 1]),
        (2097151, &[255, 255, 127]),
        (2147483647, &[255, 255, 255, 255, 7]),
        (
            9223372036854775807,
            &[255, 255, 255, 255, 255, 255, 255, 255, 127],
        ),
        (-1, &[255, 255, 255, 255, 255, 255, 255, 255, 255, 1]),
        (
            -2147483648,
            &[128, 128, 128, 128, 248, 255, 255, 255, 255, 1],
        ),
        (
            -9223372036854775808,
            &[128, 128, 128, 128, 128, 128, 128, 128, 128, 1],
        ),
    ];

    #[test]
    fn test_varlong_encode() {
        let mut vec = vec![];
        for &(number, bytes) in VARLONG_CONVERSIONS {
            VarLong(number).write(&mut vec);
            assert_eq!(vec, bytes);
            vec.clear();
        }
    }
}
