use crate::network::varint::{VarInt, VarLong};
use crate::network::{Angle, Chat, Identifier, Nbt, Slot};
use anyhow::Result;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::f32::consts::PI;
use std::io::{Read, Write};
use uuid::Uuid;

use super::Position;

pub trait MinecraftIo
where
    Self: Sized,
{
    fn minecraft_write(&self, writer: &mut impl Write) -> Result<()>;

    fn minecraft_read(reader: &mut impl Read) -> Result<Self>;
}

macro_rules! impl_minecraft_io {
    ($(($typ:ty, $read_method:ident, $write_method:ident)),* $(,)?) => {
        $(
            impl MinecraftIo for $typ {
                fn minecraft_write(&self, writer: &mut impl Write) -> Result<()> {
                    Ok(writer.$write_method::<BigEndian>(*self)?)
                }

                fn minecraft_read(reader: &mut impl Read) -> Result<Self> {
                    Ok(reader.$read_method::<BigEndian>()?)
                }
            }
        )*
    }
}

impl_minecraft_io! {
    (u16, read_u16, write_u16),
    (u32, read_u32, write_u32),
    (u64, read_u64, write_u64),
    (u128, read_u128, write_u128),
    (i16, read_i16, write_i16),
    (i32, read_i32, write_i32),
    (i64, read_i64, write_i64),
    (i128, read_i128, write_i128),
    (f32, read_f32, write_f32),
    (f64, read_f64, write_f64),
}

impl MinecraftIo for u8 {
    fn minecraft_write(&self, writer: &mut impl Write) -> Result<()> {
        Ok(writer.write_u8(*self)?)
    }

    fn minecraft_read(reader: &mut impl Read) -> Result<Self> {
        Ok(reader.read_u8()?)
    }
}

impl MinecraftIo for i8 {
    fn minecraft_write(&self, writer: &mut impl Write) -> Result<()> {
        Ok(writer.write_i8(*self)?)
    }

    fn minecraft_read(reader: &mut impl Read) -> Result<Self> {
        Ok(reader.read_i8()?)
    }
}

impl MinecraftIo for () {
    fn minecraft_write(&self, writer: &mut impl Write) -> Result<()> {
        Ok(())
    }

    fn minecraft_read(reader: &mut impl Read) -> Result<Self> {
        Ok(())
    }
}

impl MinecraftIo for bool {
    fn minecraft_write(&self, writer: &mut impl Write) -> Result<()> {
        Ok(writer.write_u8(if *self { 1 } else { 0 })?)
    }

    fn minecraft_read(reader: &mut impl Read) -> Result<Self> {
        Ok(reader.read_u8()? != 0)
    }
}

impl MinecraftIo for VarInt {
    fn minecraft_write(&self, writer: &mut impl Write) -> Result<()> {
        self.write(writer)
    }

    fn minecraft_read(reader: &mut impl Read) -> Result<Self> {
        VarInt::read(reader)
    }
}

impl MinecraftIo for VarLong {
    fn minecraft_write(&self, writer: &mut impl Write) -> Result<()> {
        Ok(self.write(writer)?)
    }

    fn minecraft_read(reader: &mut impl Read) -> Result<Self> {
        VarLong::read(reader)
    }
}

impl MinecraftIo for String {
    fn minecraft_write(&self, writer: &mut impl Write) -> Result<()> {
        VarInt(self.len() as i32).minecraft_write(writer)?;
        writer.write(self.as_bytes())?;
        Ok(())
    }

    fn minecraft_read(reader: &mut impl Read) -> Result<Self> {
        let length = VarInt::read(reader)?;
        let mut data = vec![0u8; length.0 as usize];
        reader.read(&mut data)?;
        Ok(String::from_utf8(data)?)
    }
}

const POSITION_XZ_MASK: i32 = 0x3FF_FFFF;
const POSITTION_Y_MASK: i32 = 0xFFF;

impl MinecraftIo for Position {
    fn minecraft_write(&self, writer: &mut impl Write) -> Result<()> {
        let value: u64 = ((self.x & 0x3FF_FFFF) as u64) << 38
            | ((self.z & 0x3FFFFFF) as u64) << 12
            | (self.y & 0xFFF) as u64;
        value.minecraft_write(writer)
    }

    fn minecraft_read(reader: &mut impl Read) -> Result<Self> {
        let value = <i64>::minecraft_read(reader)?;
        Ok(Position {
            x: (value >> 38) as i32,
            z: (value << 26 >> 38) as i32,
            y: (value << 52 >> 52) as i32,
        })
    }
}

impl<T: MinecraftIo> MinecraftIo for Vec<T> {
    fn minecraft_write(&self, writer: &mut impl Write) -> Result<()> {
        VarInt(self.len() as i32).minecraft_write(writer)?;
        for value in self {
            value.minecraft_write(writer)?;
        }
        Ok(())
    }

    fn minecraft_read(reader: &mut impl Read) -> Result<Self> {
        let len = <VarInt>::minecraft_read(reader)?;
        let mut vec = Vec::with_capacity(len.0 as usize);
        for _ in 0..len.0 {
            vec.push(<T>::minecraft_read(reader)?);
        }
        Ok(vec)
    }
}

impl<T: MinecraftIo> MinecraftIo for Option<T> {
    fn minecraft_write(&self, writer: &mut impl Write) -> Result<()> {
        if self.is_some() {
            self.as_ref().unwrap().minecraft_write(writer)
        } else {
            Ok(())
        }
    }

    fn minecraft_read(reader: &mut impl Read) -> Result<Self> {
        Ok(Some(<T>::minecraft_read(reader)?))
    }
}

impl MinecraftIo for Chat {
    fn minecraft_write(&self, writer: &mut impl Write) -> Result<()> {
        self.0.minecraft_write(writer)
    }

    fn minecraft_read(reader: &mut impl Read) -> Result<Self> {
        Ok(Self(<String>::minecraft_read(reader)?))
    }
}

impl MinecraftIo for Identifier {
    fn minecraft_write(&self, writer: &mut impl Write) -> Result<()> {
        self.0.minecraft_write(writer)
    }

    fn minecraft_read(reader: &mut impl Read) -> Result<Self> {
        Ok(Self(<String>::minecraft_read(reader)?))
    }
}

impl MinecraftIo for Angle {
    fn minecraft_write(&self, writer: &mut impl Write) -> Result<()> {
        let protocol_angle: u8 = (self.0 / (2. * PI) * 256.) as u8;
        protocol_angle.minecraft_write(writer)
    }

    fn minecraft_read(reader: &mut impl Read) -> Result<Self> {
        let protocol_angle = reader.read_u8()?;
        Ok(Self(protocol_angle as f32 / 256. * 2. * PI))
    }
}

impl MinecraftIo for Uuid {
    fn minecraft_write(&self, writer: &mut impl Write) -> Result<()> {
        self.as_u128().minecraft_write(writer)
    }

    fn minecraft_read(reader: &mut impl Read) -> Result<Self> {
        Ok(Uuid::from_u128(reader.read_u128::<BigEndian>()?))
    }
}

// Possibility: replace Nbt with nbt::Blob and impl this on that.
impl MinecraftIo for Nbt {
    fn minecraft_write(&self, writer: &mut impl Write) -> Result<()> {
        self.0.minecraft_write(writer)
    }

    fn minecraft_read(reader: &mut impl Read) -> Result<Self> {
        let nbt = nbt::from_reader(reader)?;
        Ok(Self(nbt))
    }
}

impl MinecraftIo for nbt::Blob {
    fn minecraft_write(&self, writer: &mut impl Write) -> Result<()> {
        let mut tmp = vec![];
        nbt::to_writer(&mut tmp, self, None)?;
        writer.write(&tmp)?;
        Ok(())
    }

    fn minecraft_read(reader: &mut impl Read) -> Result<Self> {
        let nbt = nbt::from_reader(reader)?;
        Ok(nbt)
    }
}

impl MinecraftIo for Slot {
    fn minecraft_write(&self, writer: &mut impl Write) -> Result<()> {
        match self {
            Self::Nothing => false.minecraft_write(writer),
            Self::Item { id, count, nbt } => {
                true.minecraft_write(writer)?;
                id.minecraft_write(writer)?;
                count.minecraft_write(writer)?;
                match nbt {
                    None => 0u8.minecraft_write(writer)?,
                    Some(blob) => blob.minecraft_write(writer)?,
                }
                Ok(())
            }
        }
    }

    fn minecraft_read(reader: &mut impl Read) -> Result<Self> {
        let present = <bool>::minecraft_read(reader)?;
        if !present {
            return Ok(Self::Nothing);
        }
        let id = <VarInt>::minecraft_read(reader)?;
        let count = <i8>::minecraft_read(reader)?;
        let nbt = nbt::from_reader(reader)?;
        Ok(Self::Item { id, count, nbt })
    }
}

macro_rules! tuple_impls {
    ( $( $name:ident )+ ) => {
        #[allow(non_snake_case)]
        impl<$($name:MinecraftIo),+> MinecraftIo for ($($name,)+) {
            fn minecraft_write(&self, writer: &mut impl std::io::Write) -> anyhow::Result<()> {
                let ($($name,)+) = self;
                $( $name.minecraft_write(writer)?; )+
                Ok(())
            }

            fn minecraft_read(reader: &mut impl std::io::Read) -> anyhow::Result<Self> {
                Ok((
                    $( $name::minecraft_read(reader)?, )*
                ))
            }
        }
    };
}

tuple_impls! { A }
tuple_impls! { A B }
tuple_impls! { A B C }
tuple_impls! { A B C D }
tuple_impls! { A B C D E }
tuple_impls! { A B C D E F }
tuple_impls! { A B C D E F G }
tuple_impls! { A B C D E F G H }
tuple_impls! { A B C D E F G H I }
tuple_impls! { A B C D E F G H I J }
tuple_impls! { A B C D E F G H I J K }
tuple_impls! { A B C D E F G H I J K L }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::network::Angle;

    #[test]
    fn test_write_u64() {
        let mut vec = vec![];
        64u64.minecraft_write(&mut vec);
        assert_eq!(vec, &[0, 0, 0, 0, 0, 0, 0, 64]);
    }

    #[test]
    fn test_write_string() {
        let mut vec = vec![];
        "Hello World".to_owned().minecraft_write(&mut vec);
        assert_eq!(vec, b"\x0bHello World");
    }

    #[test]
    fn test_write_position() {
        let mut vec = vec![];
        let position = Position { x: 1, y: 1, z: 1 };
        position.minecraft_write(&mut vec);
        assert_eq!(vec, &[0, 0, 0, 64, 0, 0, 16, 1]);
    }

    #[test]
    fn test_write_array() {
        let mut vec = vec![];
        let numbers = vec![2u16, 3u16, 4u16, 5u16];
        numbers.minecraft_write(&mut vec);
        assert_eq!(vec, vec![4, 0, 2, 0, 3, 0, 4, 0, 5]);
    }

    #[test]
    fn test_write_optional() {
        let mut vec = vec![];
        let mut maybe = Some(3i32);
        maybe.minecraft_write(&mut vec);
        assert_eq!(vec, &[0, 0, 0, 3]);

        vec.clear();
        maybe = None;
        maybe.minecraft_write(&mut vec);
        assert!(vec.is_empty());
    }

    #[test]
    fn test_write_angle() {
        let mut vec = vec![];
        let angle = Angle(PI);
        angle.minecraft_write(&mut vec);
        assert_eq!(vec, &[128]);
    }

    #[test]
    fn test_write_uuid() {
        let mut vec = vec![];
        let uuid = Uuid::new_v5(&Uuid::NAMESPACE_DNS, b"testing");
        // 013fad7b-475f-55b4-b2b7-0da6c41293a8
        uuid.minecraft_write(&mut vec);
        assert_eq!(
            vec,
            &[1, 63, 173, 123, 71, 95, 85, 180, 178, 183, 13, 166, 196, 18, 147, 168]
        );
    }

    #[test]
    fn test_write_simple_nbt() {
        let mut vec = vec![];
        let mut nbt = Nbt(nbt::Blob::new());
        nbt.0.insert("Key", "Value");
        nbt.minecraft_write(&mut vec);
        assert_eq!(
            vec,
            &[10, 0, 0, 8, 0, 3, 75, 101, 121, 0, 5, 86, 97, 108, 117, 101, 0]
        );
    }

    #[test]
    fn test_write_slot() {
        let mut vec = vec![];
        let mut slot = Slot::Nothing;
        slot.minecraft_write(&mut vec);
        assert_eq!(vec, &[0]);

        vec.clear();
        slot = Slot::Item {
            id: VarInt(1),
            count: 1,
            nbt: None,
        };
        slot.minecraft_write(&mut vec);
        assert_eq!(vec, &[1, 1, 1, 0]);
    }
}
