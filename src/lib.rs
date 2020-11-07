use std::net::{Ipv4Addr, ToSocketAddrs, UdpSocket};

pub struct MacAddress([u8; 6]);

pub struct Config {
    ip: Option<Ipv4Addr>,
    port: Option<usize>,
    times: Option<usize>,
}

pub struct MagicPacket([u8; 102]);

impl MacAddress {
    pub fn new(mac: String) -> std::io::Result<Self> {
        Ok(MacAddress([0; 6]))
    }
}

impl Config {
    pub fn new(ip: )
}

impl MagicPacket {
    pub fn new(mac: MacAddress) -> Self {
        MagicPacket([0; 102])
    }

    pub fn send(&self, config: Option<Config>) -> std::io::Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
