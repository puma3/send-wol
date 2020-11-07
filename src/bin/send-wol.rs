// extern crate send_wol;

use clap::Clap;
use send_wol::{MacAddress, MagicPacket};

#[derive(Clap, Debug)]
#[clap(version = "1.0", author = "Elias Puma. <eliasj.puma@outlook.com>")]
struct Opts {
    #[clap(short, long)]
    mac: String,
    ip: Option<String>,
    #[clap(short, long)]
    port: Option<i32>,
}

fn main() -> std::io::Result<()> {
    let opts: Opts = Opts::parse();

    let mac_address = MacAddress::new(opts.mac)?;
    let magic_package = MagicPacket::new(mac_address);
    magic_package.send(None)?;
    Ok(())
}
