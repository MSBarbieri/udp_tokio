#[macro_use]
extern crate text_io;

mod position;

use anyhow::Result;
use borsh::{BorshDeserialize, BorshSerialize};
use position::{Command, Player};
use std::str;
use tokio::net::UdpSocket;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Hello, client!");
    let socket = UdpSocket::bind("127.0.0.1:0").await?;
    let mut buf = [0u8; 65535];
    socket
        .send_to("new Player".as_bytes(), "127.0.0.1:6420")
        .await?;
    let len = socket.recv(&mut buf).await?;
    let player_created = str::from_utf8(&buf[..len])?;
    println!("player created {}", player_created);
    loop {
        let msg: String = read!("{}\n");
        println!("command {}", msg);
        let command = match msg.to_lowercase().as_str() {
            "w" => Command::Walk(0, 0, 1),
            "a" => Command::Walk(-1, 0, 0),
            "s" => Command::Walk(0, 0, -1),
            "d" => Command::Walk(1, 0, 0),
            _ => Command::None,
        };
        let vec = command.try_to_vec()?;
        println!("command {:?}, vec {:?}", command, vec);
        socket.send_to(&vec.as_slice(), "127.0.0.1:6420").await?;

        match socket.recv_from(&mut buf).await {
            Ok((len, addr)) => {
                println!("send command successfully");
                if addr.to_string() == "127.0.0.1:6420" {
                    let player = Player::try_from_slice(&buf[..len]);
                    println!("player {:?}", player);
                }
            }
            Err(e) => {
                println!("error ond sand command {:?}", e);
            }
        }
    }
}
