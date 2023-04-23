use std::io::{Read, Write};
use uuid::Uuid;

pub mod connection;
// mod impl_packets;
// mod packet_description;
pub mod packets;
mod read_write;
mod types;
mod varint;

pub use crate::network::read_write::MinecraftIo;
pub use packets::Packet;
pub use types::*;
pub use varint::{VarInt, VarLong};
