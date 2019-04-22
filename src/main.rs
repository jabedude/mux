use std::net::{TcpListener, TcpStream};
use std::io::Write;
use std::io::Read;
use std::time::Duration;
use std::sync::{Arc, Mutex};
use std::io::{stdout, stdin};
use std::thread;
use std::sync::RwLock;
use std::str;

use mio::*;
use mux::*;

fn mux() {
    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();
    let (mut in_stream,  addr) = listener.accept().unwrap();
    println!("new client: {:?}", addr);
    let mut out_stream = TcpStream::connect("example.com:80").unwrap();

    loop {
        let mut buf = [0u8; 1024];
        let mut resp = [0u8; 1024];
        let recv = in_stream.read(&mut buf);
        match recv {
            Ok(_) => {
                out_stream.write_all(&buf).unwrap();
                out_stream.read(&mut resp).unwrap();
                in_stream.write_all(&resp).unwrap();
                println!("recv: {}", str::from_utf8(&resp).unwrap());
            }
            Err(_) => break,
        }
    }
}

fn main() {

    // Connect to daemon
    let mut mux_stream = TcpStream::connect("127.0.0.1:8080").expect("tcp connect error");
    let mux_stream = Arc::new(Mutex::new(mux_stream));

    let mut cmds: Vec<MuxCmd> = Vec::new();

    // Start thread and Poll instance
    let poll = Arc::new(Mutex::new(Poll::new().unwrap()));

    crossbeam::scope(|scope| {
        let _handle = scope.spawn( |_| {
            let mut events = Events::with_capacity(1024);
            poll.lock().unwrap().poll(&mut events, Some(Duration::from_secs(1))).unwrap();
        });
    });

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
        let mux_data = MuxData::Cmd(cmd.clone());
        let serialized = serde_json::to_vec(&mux_data).unwrap();
        mux_stream.lock().unwrap().write(&serialized);
        cmds.push(cmd);
    }
}
