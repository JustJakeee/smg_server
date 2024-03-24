// udp game server binary
#![allow(unused)]

use std::net::UdpSocket;
use anyhow::Result;
use glam::Vec2;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use bincode;
use smg_lib::*;

struct State {
    players: HashMap<Uuid, PlayerState>,
}

fn main() -> Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:5000")?;
    println!("listening on {}", socket.local_addr()?);
    let mut state = State { players: HashMap::new() };
    let mut buf = [0; 1024];

    loop {
        let (amt, src) = socket.recv_from(&mut buf)?;
        let buf = &mut buf[..amt];
        let packet: Packet = bincode::deserialize(buf)?;
        handle_packet(packet, src, &socket, &mut state);
    }
}

fn handle_packet(packet: Packet, src: std::net::SocketAddr, socket: &UdpSocket, state: &mut State) {
    match packet {
        Packet::Connect(uuid) => {
            println!("{} connected with uuid {:?}", src, uuid);
            state.players.insert(uuid, PlayerState { uuid, x: 0., y: 0. });
            println!("{:?}", state.players);
        }
        Packet::Disconnect(uuid) => {
            println!("{} disconnected", src);
            state.players.remove(&uuid);
        }
        Packet::Message(msg) => {
            println!("message: {}", msg);
        }
        Packet::Player(player) => {
            state.players.insert(player.uuid, player);
            let players: Vec<&PlayerState> = state.players.values().collect();
            let data = bincode::serialize(&players).unwrap();
            socket.send_to(&data, &src).unwrap();
            // println!("player: {:?}", player);
        }
        Packet::List() => {
            // make a vec of all player uuids
            let uuids: Vec<Uuid> = state.players.keys().cloned().collect();
            socket.send_to(bincode::serialize(&uuids).unwrap().as_slice(), &src).unwrap();
            // println!("prompted list of players to {:?}", src);
        }
        _ => {
            println!("unknown packet");
        }
    }
}