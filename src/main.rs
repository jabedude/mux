use std::net::{TcpListener, TcpStream};
use std::io::Write;
use std::io::Read;
use std::io::{stdout, stdin};
use std::str;

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
    // let mut mux_tream = TcpStream::connect("127.0.0.1:8080").expect("Tcp Connect error");

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
        let mut cmd: MuxCmd = match input.parse() {
            Ok(c) => c,
            Err(e) => {
                println!("Invalid command: {}", e);
                continue;
            }
        };
        cmd.mux_id = cmds.len() + 1;
        cmds.push(cmd);
    }
}
