use std::str::FromStr;
use std::num::ParseIntError;
use std::net::IpAddr;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct MuxData {
    pub mux_id: usize,
    pub data: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MuxCmd {
    //pub mux_id: usize,
    pub lport: u32,
    pub dport: u32,
    pub dest_ip: IpAddr,
}

impl FromStr for MuxCmd {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let input: Vec<&str> = s.split(":").collect();
        println!("input: {:?}", input);
        let lport = input[0].parse::<u32>()?;
        println!("lport: {:?}", lport);
        let dport = input[2].parse::<u32>()?;
        println!("dport: {:?}", dport);
        let dest_ip: IpAddr = input[1].parse().expect("Ip Addr failed"); // TODO: fix

        Ok (
            MuxCmd {lport: lport, dport: dport, dest_ip: dest_ip}
        )
    }
}
