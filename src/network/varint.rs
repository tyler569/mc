use std::io::{Read, Write};

const SEGMENT_BITS: u32 = 0x7f;
const CONTINUE_BIT: u8 = 0x80;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct VarInt(i32);

impl VarInt {
    pub fn get(self) -> i32 {
        self.0
    }

    pub fn write(self, writer: &mut dyn Write) {
        let mut buffer = [0u8; 6];
        let mut index = 0;
        let mut value = self.0 as u32;
        loop {
            if value & !SEGMENT_BITS == 0 {
                buffer[index] = value as u8;
                index += 1;
                writer.write(&buffer[0..index]);
                return;
            }

            buffer[index] = value as u8 | CONTINUE_BIT;
            index += 1;
            value >>= 7;
        }
    }
}

#[cfg(test)]
mod tests {
    const CONVERSIONS: &[(i32, &[u8])] = &[
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

    #[test]
    fn test_varint_encode() {
        let mut vec = vec![];
        for &(number, bytes) in CONVERSIONS {
            VarInt(number).write(&mut vec);
            assert_eq!(vec, bytes);
            vec.clear();
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct VarLong(i64);

impl VarLong {
    pub fn get(self) -> i64 {
        self.0
    }
}
