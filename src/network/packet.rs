use std::io::{Result, Write};
use crate::network::varint::{VarInt, VarLong};

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

impl_minecraft_write!(i8, i16, i32, i64, u8, u16, u32, u64, f32, f64);

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