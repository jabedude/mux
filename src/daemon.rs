use std::net::TcpListener;
use std::net::TcpStream;
use std::io::Write;
use std::io::Read;
use std::collections::HashMap;
use std::str;

use mux::*;

fn main() {
    // Vector of Tcp connections
    let mut mux_connections: HashMap<usize, TcpStream> = HashMap::new();

    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();
    let (mut mux_stream,  addr) = listener.accept().unwrap();
    println!("new client: {:?}", addr);
    loop {
        let mut buf: Vec<u8> = vec![0u8; 8192];
        let recv = mux_stream.read(&mut buf).unwrap();
        let deserialized: MuxData = serde_json::from_slice(&buf[..recv]).expect("serde deserialize err");
        match deserialized {
            MuxData::Tx(tx) => {
                println!("{:?}", tx);
                let mut mux = mux_connections.get(&tx.mux_id).unwrap();
                mux.write(&tx.data);
            }
            MuxData::Cmd(cmd) => {
                println!("{:?}", cmd);
                let mut mux = TcpStream::connect((cmd.dest_ip, cmd.dport)).unwrap();
                println!("tcp established");
                mux_connections.insert(cmd.mux_id, mux);
                println!("tcp pushed");
            }
        }
    }
}
