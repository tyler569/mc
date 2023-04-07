use crate::network::varint::{VarInt, VarLong};
use crate::network::{Angle, Chat, Identifier, Nbt, Slot};
use std::f32::consts::PI;
use std::io::{Result, Write};
use uuid::Uuid;

use super::Position;

trait MinecraftWrite {
    fn minecraft_write(&self, writer: &mut dyn Write) -> Result<usize>;
}

macro_rules! impl_minecraft_write {
    ($($typ:ty),*) => {
        $(
            impl MinecraftWrite for $typ {
                fn minecraft_write(&self, writer: &mut dyn Write) -> Result<usize> {
                    writer.write(&self.to_be_bytes())
                }
            }
        )*
    };
}

impl_minecraft_write!(i8, i16, i32, i64, u8, u16, u32, u64, u128, f32, f64);

impl MinecraftWrite for bool {
    fn minecraft_write(&self, writer: &mut dyn Write) -> Result<usize> {
        writer.write(&[*self as u8])
    }
}

impl MinecraftWrite for VarInt {
    fn minecraft_write(&self, writer: &mut dyn Write) -> Result<usize> {
        self.write(writer)
    }
}

impl MinecraftWrite for VarLong {
    fn minecraft_write(&self, writer: &mut dyn Write) -> Result<usize> {
        self.write(writer)
    }
}

impl MinecraftWrite for &str {
    fn minecraft_write(&self, writer: &mut dyn Write) -> Result<usize> {
        let len = self.len() as i32;
        let mut size = VarInt(len).minecraft_write(writer)?;
        size += writer.write(self.as_bytes())?;
        Ok(size)
    }
}

impl MinecraftWrite for Position {
    fn minecraft_write(&self, writer: &mut dyn Write) -> Result<usize> {
        let value: u64 = ((self.x & 0x3FFFFFF) as u64) << 38
            | ((self.z & 0x3FFFFFF) as u64) << 12
            | (self.y & 0xFFF) as u64;
        value.minecraft_write(writer)
    }
}

// Can't do this because it conflicts with &[T], but I can always just .write(&[u8]).
//
// impl MinecraftWrite for &[u8] {
//     fn minecraft_write(&self, writer: &mut dyn Write) -> Result<usize> {
//         writer.write(self)
//     }
// }

impl<T: MinecraftWrite> MinecraftWrite for &[T] {
    fn minecraft_write(&self, writer: &mut dyn Write) -> Result<usize> {
        let mut size = 0;
        for v in *self {
            size += v.minecraft_write(writer)?;
        }
        Ok(size)
    }
}

impl<T: MinecraftWrite> MinecraftWrite for Option<T> {
    fn minecraft_write(&self, writer: &mut dyn Write) -> Result<usize> {
        if self.is_some() {
            self.as_ref().unwrap().minecraft_write(writer)
        } else {
            Ok(0)
        }
    }
}

impl MinecraftWrite for Chat {
    fn minecraft_write(&self, writer: &mut dyn Write) -> Result<usize> {
        self.0.as_str().minecraft_write(writer)
    }
}

impl MinecraftWrite for Identifier {
    fn minecraft_write(&self, writer: &mut dyn Write) -> Result<usize> {
        self.0.as_str().minecraft_write(writer)
    }
}

impl MinecraftWrite for Angle {
    fn minecraft_write(&self, writer: &mut dyn Write) -> Result<usize> {
        let protocol_angle: u8 = (self.0 / (2. * PI) * 256.) as u8;
        protocol_angle.minecraft_write(writer)
    }
}

impl MinecraftWrite for Uuid {
    fn minecraft_write(&self, writer: &mut dyn Write) -> Result<usize> {
        self.as_u128().minecraft_write(writer)
    }
}

// Possibility: replace Nbt with nbt::Blob and impl this on that.
impl MinecraftWrite for Nbt {
    fn minecraft_write(&self, writer: &mut dyn Write) -> Result<usize> {
        let mut tmp = vec![];
        nbt::to_writer(&mut tmp, &self.0, None)?;
        writer.write(&tmp)
    }
}

impl MinecraftWrite for nbt::Blob {
    fn minecraft_write(&self, writer: &mut dyn Write) -> Result<usize> {
        let mut tmp = vec![];
        nbt::to_writer(&mut tmp, self, None)?;
        writer.write(&tmp)
    }
}

impl MinecraftWrite for Slot {
    fn minecraft_write(&self, writer: &mut dyn Write) -> Result<usize> {
        match self {
            Self::Nothing => false.minecraft_write(writer),
            Self::Item { id, count, nbt } => {
                let mut size = 0;
                size += true.minecraft_write(writer)?;
                size += id.minecraft_write(writer)?;
                size += count.minecraft_write(writer)?;
                match nbt {
                    None => size += 0u8.minecraft_write(writer)?,
                    Some(blob) => size += blob.minecraft_write(writer)?,
                }
                Ok(size)
            }
        }
    }
}

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
        "Hello World".minecraft_write(&mut vec);
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
        let numbers: &[u16] = &[2u16, 3u16, 4u16, 5u16];
        numbers.minecraft_write(&mut vec);
        assert_eq!(vec, &[0, 2, 0, 3, 0, 4, 0, 5]);
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
