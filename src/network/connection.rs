use crate::network::{ServerBoundHandshakePacket, VarInt};
use std::net::TcpStream;

pub fn connect_to_server(address: &str, port: u16) -> anyhow::Result<()> {
    // let stream = TcpStream::connect((address, port))?;

    let packet = ServerBoundHandshakePacket::Handshake(super::serverbound::handshake::Handshake {
        protocol_version: VarInt(758),
        server_address: address.to_owned(),
        server_port: port,
        next_state: VarInt(1),
    });

    let mut buffer: Vec<u8> = vec![];

    packet.write(&mut buffer);

    println!("{:x?}", buffer);

    Ok(())
}
