use std::net::{TcpStream, TcpListener};
use std::io::Write;
use std::io::Read;
use std::time::Duration;
use std::sync::{Arc, Mutex};
use std::io::{stdout, stdin};
use std::thread;
use std::sync::RwLock;
use std::str;
#[macro_use]
extern crate log;

use mux::*;

fn main() {
    env_logger::init();
    // Connect to daemon
    let mut mux_stream = TcpStream::connect("127.0.0.1:8080").expect("tcp connect error");
    let mux_stream = Arc::new(Mutex::new(mux_stream));

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

            // Set up local port and thread
            // TODO: fix this mess
            // TODO: timeout?
            let id = cmd.mux_id;
            let port = cmd.lport;
            let mux = Arc::clone(&mux_stream);
            let _handle = thread::spawn( move || {
                let listener = TcpListener::bind(&format!("0.0.0.0:{}", port)).unwrap();
                let (mut sock, client) = listener.accept().unwrap();
                let mut buf: Vec<u8> = vec![0u8; 1024];
                loop {
                    match sock.read(&mut buf) {
                        Ok(0) => continue,
                        Ok(_) => {
                            let mux_tx = MuxTx::new(id, buf.clone());
                            let mux_data = MuxData::Tx(mux_tx);
                            let serialized = serde_json::to_vec(&mux_data).unwrap();
                            println!("serialized len: {}", serialized.len());
                            mux.lock().unwrap().write(&serialized);
                        }
                        Err(e) => println!("Error: {:?}", e),
                    }
                }
            });

            // Send Mux data to daemon
            let mux_data = MuxData::Cmd(cmd.clone());
            let serialized = serde_json::to_vec(&mux_data).unwrap();
            mux_stream.lock().unwrap().write(&serialized);
            cmds.push(cmd);
        }
}
