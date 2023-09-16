use std::{net::{IpAddr, Ipv4Addr, SocketAddr}, time::Duration};

use clap::Parser;
use ll_udp_pubsub::{publisher::UdpPublisher, timestamp};

use std::fs;
//use std::io::prelude::*;
use std::io::{self,BufReader, Read};

#[derive(clap::Parser)]
/// Send counter with specified timeout
struct Cmd {
    /// Address to bind publisher to
    #[clap(short = 's')]
    server_addr: SocketAddr,
    /// Address where client is listening
    #[clap(short = 'c')]
    client_addr: SocketAddr,
    /// Timeout to send messages in microseconds
    #[clap(short = 't', default_value = "1000")]
    timeout_micros: u64,
    /// Number of messages to send
    #[clap(short = 'n', default_value = "100")]
    number: usize,
    /// Pin sender to core
    #[arg(long = "core", env = "SENDER_CORE")]
    core: Option<usize>,
}

// cargo run --bin fast-udp-sender -- -s 127.0.0.1:12345 -c 127.0.0.1:34254
fn main() {
/*
    let opts = Cmd::parse();
    let mut publisher = UdpPublisher::new(opts.server_addr).unwrap();
    publisher.set_nonblocking(true).unwrap();
    if let Some(core) = opts.core {
        ll_udp_pubsub::pin_to_core(core);
    }
    let recipients = vec![opts.client_addr];
    let timeout = Duration::from_micros(opts.timeout_micros);
    for i in 1..=opts.number {
        publisher.send((i, timestamp()), recipients.iter()).unwrap();
        std::thread::sleep(timeout);
    }
*/

let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 12345);
let mut publisher = UdpPublisher::new(socket).unwrap();
publisher.set_nonblocking(true).unwrap();
ll_udp_pubsub::pin_to_core(1);
let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 34254);
let recipients:Vec<SocketAddr> = vec![socket];
let timeout = Duration::from_micros(1000u64);

let f = fs::File::open("source/video.mp4").unwrap();
let mut reader = BufReader::new(f);
let mut buffer = [0u8; 1470];
let mut count_send = 0;
loop{
    if let Ok(n) = reader.read(&mut buffer[..]){
        if n == 0{
            break;
        }
       
        publisher.send((&buffer[..n], timestamp()) , recipients.iter()).unwrap();
        std::thread::sleep(timeout);
        count_send+=1;
    }else{
        println!("Err");
        break;
    }
}
println!("count_send={count_send}");// count_send=33630
}