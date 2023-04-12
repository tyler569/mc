use crate::network::{ServerBoundHandshakePacket, VarInt};
use std::net::TcpStream;

fn connect_to_server(address: &str, port: u16) -> anyhow::Result<()> {
    let stream = TcpStream::connect((address, port))?;

    let packet = ServerBoundHandshakePacket::Handshake {
        protocol_version: VarInt(758),
        server_address: address.to_owned(),
        server_port: port,
        next_state: VarInt(1),
    };

    let buffer: Vec<u8> = vec![];

    Ok(())
    // stream.write
}
