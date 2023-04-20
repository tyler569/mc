use crate::network::{packets::*, MinecraftIo, *};
use anyhow::Result;
use std::fmt::Display;
use std::io::{Read, Write};

pub trait PacketIo where Self: Sized {
    fn minecraft_write(&self, writer: &mut impl Write, protocol_version: i32) -> Result<()>;

    fn minecraft_read(reader: &mut impl Read, protocol_version: i32) -> Result<Self>;
}

impl PacketIo for Handshake {
    fn minecraft_write(&self, writer: &mut impl Write, _: i32) -> Result<()> {
        self.protocol_version.minecraft_write(writer)?;
        self.server_address.minecraft_write(writer)?;
        self.server_port.minecraft_write(writer)?;
        self.protocol_version.minecraft_write(writer)?;
        Ok(())
    }

    fn minecraft_read(reader: &mut impl Read, _: i32) -> Result<Self> {
        Ok(Self {
            protocol_version: VarInt::minecraft_read(reader)?,
            server_address: String::minecraft_read(reader)?,
            server_port: u16::minecraft_read(reader)?,
            next_state: VarInt::minecraft_read(reader)?,
        })
    }
}

impl PacketIo for StatusResponse {
    fn minecraft_write(&self, writer: &mut impl Write, _: i32) -> Result<()> {
        self.json_data.minecraft_write(writer)?;
        Ok(())
    }

    fn minecraft_read(reader: &mut impl Read, _: i32) -> Result<Self> {
        Ok(Self {
            json_data: String::minecraft_read(reader)?,
        })
    }
}

impl PacketIo for StatusPong {
    fn minecraft_write(&self, writer: &mut impl Write, _protocol_version: i32) -> Result<()> {
        self.payload.minecraft_write(writer)?;
        Ok(())
    }

    fn minecraft_read(reader: &mut impl Read, _protocol_version: i32) -> Result<Self> {
        Ok(Self {
            payload: i64::minecraft_read(reader)?,
        })
    }
}

impl PacketIo for StatusRequest {
    fn minecraft_write(&self, writer: &mut impl Write, protocol_version: i32) -> Result<()> {
        Ok(())
    }

    fn minecraft_read(reader: &mut impl Read, protocol_version: i32) -> Result<Self> {
        Ok(Self {})
    }
}

impl PacketIo for StatusPing {
    fn minecraft_write(&self, writer: &mut impl Write, protocol_version: i32) -> Result<()> {
        self.payload.minecraft_write(writer)?;
        Ok(())
    }

    fn minecraft_read(reader: &mut impl Read, protocol_version: i32) -> Result<Self> {
        Ok(Self {
            payload: i64::minecraft_read(reader)?,
        })
    }
}

impl PacketIo for Disconnect {
    fn minecraft_write(&self, writer: &mut impl Write, protocol_version: i32) -> Result<()> {
        self.reason.minecraft_write(writer)?;
        Ok(())
    }

    fn minecraft_read(reader: &mut impl Read, protocol_version: i32) -> Result<Self> {
        Ok(Self {
            reason: String::minecraft_read(reader)?,
        })
    }
}
