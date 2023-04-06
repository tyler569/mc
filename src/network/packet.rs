use std::io::Write;
use crate::network::varint::{VarInt, VarLong};

use super::Position;

pub struct PacketEncoder {
    data: Vec<u8>,
}

// macro_rules! trivial_write {
//     ($($type:ty),*) => {
//         $(
//             pub fn write_$type(&mut self, value: $type) {
//                 self.data.write(&value.to_be_bytes);
//             }
//         )*
//     };
// }

impl PacketEncoder {
    pub fn new() -> Self {
        Self {
            data: vec![],
        }
    }

    pub fn write_i8(&mut self, value: i8) {
        self.data.push(value as u8);
    }

    pub fn write_i16(&mut self, value: i16) {
        self.data.write(&value.to_be_bytes());
    }

    pub fn write_i32(&mut self, value: i32) {
        self.data.write(&value.to_be_bytes());
    }

    pub fn write_i64(&mut self, value: i64) {
        self.data.write(&value.to_be_bytes());
    }

    pub fn write_u8(&mut self, value: u8) {
        self.data.push(value);
    }

    pub fn write_u16(&mut self, value: u16) {
        self.data.write(&value.to_be_bytes());
    }

    pub fn write_u32(&mut self, value: u32) {
        self.data.write(&value.to_be_bytes());
    }

    pub fn write_u64(&mut self, value: u64) {
        self.data.write(&value.to_be_bytes());
    }

    pub fn write_f32(&mut self, value: f32) {
        self.data.write(&value.to_be_bytes());
    }

    pub fn write_f64(&mut self, value: f64) {
        self.data.write(&value.to_be_bytes());
    }


    pub fn write_varint(&mut self, value: VarInt) {
        value.write(&mut self.data);
    }

    pub fn write_varlong(&mut self, value: VarLong) {
        value.write(&mut self.data);
    }

    pub fn write_position(&mut self, position: Position) {
        let v = (((position.x & 0x3FFFFFF << 38) as i64) |
            ((position.z & 0x3FFFFFF << 12) as i64) |
            ((position.y & 0xFFFF) as i64)) as u64;
        self.write_u64(v);
    }
}