use std::net::{TcpStream};
use mio::net::{TcpListener};
use std::io::Write;
use std::io::Read;
use std::time::Duration;
use std::sync::{Arc, Mutex};
use std::io::{stdout, stdin};
use std::thread;
use std::sync::RwLock;
use std::collections::HashMap;
use std::str;
#[macro_use]
extern crate log;

use crossbeam::channel;
use mio::*;
use mux::*;

fn main() {
    env_logger::init();
    // Connect to daemon
    let mut mux_stream = TcpStream::connect("127.0.0.1:8080").expect("tcp connect error");
    let mux_stream = Arc::new(Mutex::new(mux_stream));

    // Create TX/RX endpoints
    let (tx, rx) = channel::bounded(1);

    // Start thread and Poll instance
    let poll = Arc::new(Mutex::new(Poll::new().unwrap()));

    crossbeam::scope(|scope| {
        let _handle = scope.spawn( |_| {
            let mut events = Events::with_capacity(10);
            let mut server_map: HashMap<Token, TcpListener> = HashMap::new();
            loop {
                let p = poll.lock().unwrap();
                info!("recieving server...");
                match rx.try_recv() {
                    Ok(server) => {
                        let token = Token(server_map.len() + 1);
                        p.register(&server, token, Ready::readable(), PollOpt::edge()).unwrap();
                        server_map.insert(token, server);
                    },
                    Err(_) => (),
                };
                info!("thread polling...");
                p.poll(&mut events, Some(Duration::from_secs(1))).unwrap();

		for event in events.iter() {
                    let t = event.token();
		    let server = server_map.get(&t).unwrap();
                    let (sock, _) = server.accept().unwrap();
                    p.register(&sock, Token(server_map.len() + 1), Ready::readable(), PollOpt::edge()).expect("register error");
                }
            }
        });

        let mut cmds: Vec<MuxCmd> = Vec::new();

        loop {
            for c in &cmds {
                println!("{:?}", c);
            }
            print!("mux > ");
            stdout().flush();
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();
            input.pop(); // Pop newline..TODO: maybe a better way?

            if input == "help" || input == "?" {
                println!("help menu TODO");
                continue;
            }

            let mut cmd: MuxCmd = match input.parse() {
                Ok(c) => c,
                Err(e) => {
                    println!("Invalid command: {}", e);
                    continue;
                }
            };

            // Hacky, set the mux_id of the cmd by using the len of the cmd vec
            cmd.mux_id = cmds.len() + 1;

            // Set up local port
            // TODO: fix this mess
            let server = TcpListener::bind(&format!("0.0.0.0:{}", &cmd.lport).parse().unwrap()).unwrap();
            tx.send(server).unwrap();

            // Send Mux data to daemon
            let mux_data = MuxData::Cmd(cmd.clone());
            let serialized = serde_json::to_vec(&mux_data).unwrap();
            mux_stream.lock().unwrap().write(&serialized);
            cmds.push(cmd);
        }
    });
}
