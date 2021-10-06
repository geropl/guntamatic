pub mod stream;
pub mod get;

use std::net::IpAddr;


use clap::Clap;

#[derive(Clap)]
#[derive(Clone)]
pub struct Options {
    /// The IP address of the local network device to bind to. ex.: 127.0.0.1
    #[clap(
        long = "iface",
        name = "if",
        global = true,
        parse(try_from_str = parse_ip_addr)
    )]
    pub iface_addr: Option<IpAddr>,

    /// The address/IP of the Guntamatic device to stream data from
    #[clap()]
    pub addr: String,

    /// The key to authenticate with against the device
    #[clap()]
    pub key: String,


    #[clap(subcommand)]
    pub cmd: SubCmds,
}

#[derive(Clap)]
#[derive(Clone)]
pub enum SubCmds {
    #[clap(
        name = "stream",
        about = "Stream DAQ data to one of various sinks"
    )]
    Stream(stream::Options),
    #[clap(
        name = "get",
        about = "Get DAQ data and print it to stdout"
    )]
    Get(get::Options),
}

fn parse_ip_addr(addr: &str) -> Result<IpAddr, std::net::AddrParseError> {
    addr.parse()
}
