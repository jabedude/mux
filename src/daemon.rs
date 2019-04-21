use std::net::TcpListener;
use std::net::TcpStream;
use std::io::Write;
use std::io::Read;
use std::str;

use mux::*;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();
    let (mut mux_stream,  addr) = listener.accept().unwrap();
    println!("new client: {:?}", addr);
    let mut buf = vec![0u8; 1024];

    loop {
        let recv = mux_stream.read(&mut buf);
    }
}
