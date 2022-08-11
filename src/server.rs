use anyhow::Result;
mod position;
use borsh::{BorshDeserialize, BorshSerialize};
use position::{Command, Player, Position};
use std::{collections::hash_map::HashMap, str};
use tokio::net::UdpSocket;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Hello, server!");
    let socket = UdpSocket::bind("127.0.0.1:6420").await?;
    let mut buf = [0u8; 65535];
    let mut players: HashMap<String, Player> = HashMap::new();

    loop {
        match socket.recv_from(&mut buf).await {
            Ok((l, addr)) => {
                let payload = match Command::try_from_slice(&buf[..l]) {
                    Ok(c) => {
                        let player = players.get_mut(&addr.to_string());

                        match c {
                            // w
                            Command::Walk(0, 0, 1) => {
                                player.unwrap().position.z += 1;
                            }
                            // a
                            Command::Walk(-1, 0, 0) => {
                                player.unwrap().position.x -= 1;
                            }

                            // s
                            Command::Walk(0, 0, -1) => {
                                player.unwrap().position.z -= 1;
                            }
                            // d
                            Command::Walk(1, 0, 0) => {
                                player.unwrap().position.x += 1;
                            }
                            Command::Walk(_, _, _) => (),
                            Command::Shot => (),
                            Command::None => (),
                        };

                        players.get_mut(&addr.to_string())
                    }
                    Err(_) => match str::from_utf8(&buf[..l]) {
                        Ok(s) if s == "new Player" => {
                            if !players.contains_key(&addr.to_string()) {
                                let player = Player {
                                    id: 1,
                                    position: Position { x: 0, y: 0, z: 0 },
                                };
                                players.insert(addr.to_string(), player);
                                println!("player created, {:?}", players);
                                players.get_mut(&addr.to_string())
                            } else {
                                players.get_mut(&addr.to_string())
                            }
                        }
                        Ok(s) => {
                            println!("something strange {}", s);
                            None
                        }
                        Err(e) => {
                            panic!("error on parse packet, {:?}", e);
                        }
                    },
                };

                socket
                    .send_to(payload.unwrap().try_to_vec()?.as_slice(), addr)
                    .await?;
            }
            Err(e) => {
                panic!("error on recv packet, {:?}", e);
            }
        }
    }
}
