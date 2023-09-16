
use std::net::UdpSocket;
use std::net::SocketAddr;

use std::fs;
use std::io;
use std::io::prelude::*;
use std::io::{BufReader};

// cargo run --bin udp-server <server> 
// cargo run --bin udp-server localhost:34254
fn main() -> std::io::Result<()>{
 
    // Settings
    let mut base_server_addr:String = "127.0.0.1:34254".to_string();
    if std::env::args().len() > 1 {
         let args: Vec<String> = std::env::args().collect(); 
         base_server_addr = args[1].clone();
    }
    println!("\nRun listen addr: {}\n",ansi_term::Colour::Red.bold().paint(&base_server_addr)); 
    
     // Create socket
    let socket = UdpSocket::bind(base_server_addr).expect("failed to bind host socket");
    const SIZE_DATAGRAM_MAX:usize = 65535-32;
    let mut buf = [0; SIZE_DATAGRAM_MAX];

    // Listenings ...
    loop {
        let sock = socket.try_clone()?;
        println!("blocking");
        // RX
        match socket.recv_from(&mut buf) {
            Ok((number_of_bytes_read, SocketAddr::V4(client_addr))) => {
                std::thread::spawn(move || {
                    let buf = &mut buf[..number_of_bytes_read];
                    print!("Client addr {} ",  &client_addr);
                    print!("Read {} bytes ", number_of_bytes_read);
                    //println!("with msg:{}", String::from_utf8(buf.to_vec()).unwrap());
                    // TX
                    //buf.reverse();
                    //sock.send_to(&buf, &client_addr).expect("error sending");
                    //----------------------------------------------------------------
                    let f = fs::File::open("source/video.mp4").unwrap();
                    let mut reader = BufReader::new(f);
                    let mut buffer = [0; 4096*2];
                
                    loop{println!(".");
                        if let Ok(n) = reader.read(&mut buffer[..]){
                            println!("N:{n}");
                            sock.send_to(&buffer/*[..n]*/, &client_addr).expect("error sending");
                        }else{
                            println!("Err");
                            break;
                        }
                    }
                    //----------------------------------------------------------------

                });
            },
            Ok((_, SocketAddr::V6(_))) => { eprintln!("Used only IPv4");},
            Err(err) => {
                eprintln!("Err: {}", err);
            }
        }
    }
}
// udp://192.168.1.77:34254
// udp://127.0.0.1:34254