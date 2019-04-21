use std::fmt;
use std::str::FromStr;
use std::num::ParseIntError;
use std::net::IpAddr;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum MuxData {
    Tx(MuxTx),
    Cmd(MuxCmd),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MuxTx {
    pub mux_id: usize,
    pub data: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MuxCmd {
    pub mux_id: usize,
    pub lport: u32,
    pub dport: u32,
    pub dest_ip: IpAddr,
}

pub enum Error {
    ParseError,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            ParseError => {
                write!(f, "{}", "ParseError")
            }
        }
    }
}

impl From<ParseIntError> for Error {
    fn from(_: ParseIntError) -> Self {
        Error::ParseError
    }
}
impl From<std::net::AddrParseError> for Error {
    fn from(_: std::net::AddrParseError) -> Self {
        Error::ParseError
    }
}

impl FromStr for MuxCmd {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let input: Vec<&str> = s.split(":").collect();
        let lport = input[0].parse::<u32>()?;
        println!("lport: {}", lport);
        let dport = input[2].parse::<u32>()?;
        println!("dport: {}", dport);
        let dest_ip: IpAddr = input[1].parse()?; // TODO: fix

        Ok (MuxCmd {
                mux_id: 0usize,
                lport: lport,
                dport: dport,
                dest_ip: dest_ip
            })
    }
}
