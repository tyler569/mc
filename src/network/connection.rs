use crate::network::{packets, Packet, VarInt};
use std::io::{BufReader, BufWriter, Read, Write};
use std::net::TcpStream;
use winit::event::VirtualKeyCode::S;

enum Direction {
    ServerBound,
    ClientBound,
}

enum State {
    Handshake,
    Status,
    Login,
    Play,
}

struct Connection {
    direction: Direction,
    state: State,

    reader: Box<dyn Read>,
    writer: Box<dyn Write>,
}

pub fn connect_to_server(address: &str, port: u16) -> anyhow::Result<()> {
    let stream = TcpStream::connect((address, port))?;
    let reader = Box::new(BufReader::new(stream.try_clone().unwrap()));
    let writer = Box::new(stream);

    let conn = Connection {
        direction: Direction::ClientBound,
        state: State::Handshake,

        reader,
        writer,
    };

    Ok(())
}

pub fn test_handshake(address: &str, port: u16) {
    let packet = Packet::Handshake(packets::Handshake {
        protocol_version: VarInt(758),
        server_address: address.to_owned(),
        server_port: port,
        next_state: VarInt(1),
    });

    let mut buffer: Vec<u8> = vec![];
    // packet.write_to(&mut buffer);
    println!("{:x?}", buffer);
}

pub fn write_packet() -> anyhow::Result<()> {
    Ok(())
}

pub fn read_packet() -> anyhow::Result<()> {
    Ok(())
}
