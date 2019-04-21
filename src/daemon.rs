use std::net::TcpListener;
use std::net::TcpStream;
use std::io::Write;
use std::io::Read;
use std::str;

fn main() {
    println!("Hello, world!");
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
