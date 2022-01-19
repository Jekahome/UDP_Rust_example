use std::net::UdpSocket;

// cargo run --bin udp-client <client> <server>

// cargo run --bin udp-client localhost:12345 localhost:34254
fn main() -> std::io::Result<()>{

    // Settings
    let mut base_client_addr:String = "localhost:12345".to_string();
    if std::env::args().len() > 1 {
        let args: Vec<String> = std::env::args().collect(); 
        base_client_addr = args[1].clone();
    }
    let mut server_addr:String = "localhost:34254".to_string();
    if std::env::args().len() > 2 {
        let args: Vec<String> = std::env::args().collect(); 
        server_addr = args[2].clone();
    }
    println!("\nClient {} send to server {}\n",
    ansi_term::Colour::Red.bold().paint(&base_client_addr),
    ansi_term::Colour::Red.bold().paint(&server_addr));

    // Create socket
    let socket = UdpSocket::bind(base_client_addr)?;
  
    // Without connect

        // TX
        let mut data = "Hello".as_bytes().to_vec();
        while match socket.send_to(&data, &server_addr){
          Ok(write_bytes) if write_bytes == data.len()  =>{
            println!("Data sent in full");
            false
          },
        _ =>{
            eprintln!("Couldn't send data");
            true
          }
        }{}
        
        // RX
        let mut buf = [0; 10];
        let (number_of_bytes_read, src_addr) = socket.recv_from(&mut buf).expect("Didn't receive data");
        let answer_server = &mut buf[..number_of_bytes_read];

        println!("Server addr {:?} \nAnswer {:?}",src_addr, String::from_utf8(answer_server.to_vec()).unwrap());

        data.reverse();
        assert_eq!(data ,answer_server);
     
    // With connect
   
        // TX
        socket.connect(&server_addr).expect("couldn't connect to address");
        const SIZE_DATAGRAM_MAX:usize = 65535-32;
        let mut data = vec![78;SIZE_DATAGRAM_MAX];//"Hello".as_bytes().to_vec();
        data[0]=1;
        while match socket.send(&data){
          Ok(write_bytes) if write_bytes == data.len()  =>{
            println!("Data sent in full");
            false
          },
        _ =>{
            eprintln!("Couldn't send data");
            true
          }
        }{}

        // RX
        let mut buf = [0; SIZE_DATAGRAM_MAX]; 
        match socket.recv(&mut buf) {
          Ok(number_of_bytes_read) => {
            //println!("received {} bytes {:?}", number_of_bytes_read, &buf[..number_of_bytes_read]);
             let answer_server = &mut buf[..number_of_bytes_read];
             println!("Server addr {:?}",server_addr);
             println!("Read {} bytes",number_of_bytes_read);
             //println!("Answer {:?}", String::from_utf8(answer_server.to_vec()).unwrap());
             data.reverse();
             assert_eq!(data ,answer_server);
          },
          Err(e) => {
            println!("recv function failed: {:?}", e);
        }
        }
 Ok(())
}