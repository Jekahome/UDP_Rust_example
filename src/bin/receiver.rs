use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
    time::Duration,
};

use clap::Parser;
use ll_udp_pubsub::{
    subscriber::{UdpSubscriber, UdpSubscriberConfig},
    Handler, Packet,
};

use std::fs;
use std::io;
use std::io::prelude::*;
use std::io::{BufWriter};

#[derive(clap::Parser)]
/// Receive counters with specified timeout
struct Cmd {
    /// Address where client is listening
    #[arg(short = 'c')]
    client_addr: SocketAddr,
    /// Number of messages to receive
    #[arg(short = 'n', default_value = "100")]
    number: usize,
    /// Non-blocking receive
    #[arg(long = "non-blocking")]
    non_blocking: bool,
    /// Pin subscriber to core
    #[arg(long = "core", env = "RECEIVER_CORE")]
    core: Option<usize>,
}

struct Msg {
    id: Vec<u8>,//[u8;100],// i64
    sent_ts: i64,
    received_ts: i64,
}

#[derive(Default, Clone)]
struct Receiver {
    messages: Arc<Mutex<Vec<Msg>>>,
}

impl Handler for Receiver {
    type Message = Vec<u8>;// i64;

    fn handle(&mut self, msg: Packet<Self::Message>, received_ts: i64) {
        self.messages.lock().unwrap().push(Msg {
            id: msg.data,
            sent_ts: msg.sent_ts,
            received_ts,
        })
    }
}

// cargo run --bin fast-udp-receiver -- -c 127.0.0.1:34254 -n 33630
fn main() {
    let opts = Cmd::parse();
    let subscriber_config = UdpSubscriberConfig::new(opts.client_addr);
    let receiver = Receiver::default();
    let mut subscriber = UdpSubscriber::new(subscriber_config, receiver.clone()).unwrap();
    subscriber.set_nonblocking(opts.non_blocking).unwrap();
    let subscriber_handle = subscriber.spawn(opts.core).unwrap();

    // Wait until client receives expected number of messages
    while receiver.messages.lock().unwrap().len() < opts.number {
        std::thread::sleep(Duration::from_millis(1_000));
    }

    // Stop subscriber
    subscriber_handle.shutdown().unwrap();

    let f = fs::File::create("source/new_video.mp4").unwrap();
    let mut writer = BufWriter::new(f);

    // Print out csv with results
    println!("id,sent_ts,received_ts");
    for Msg {
        id,
        sent_ts,
        received_ts,
    } in receiver.messages.lock().unwrap().iter() {
        println!("{:?},{sent_ts},{received_ts}",id.len());
        writer.write(id).unwrap();
    }
}