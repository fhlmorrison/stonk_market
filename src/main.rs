use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream, UdpSocket},
};

mod engine;

fn handle_client(mut stream: TcpStream) {
    let mut buffer: [u8; 1024] = [0; 1024];

    stream.read(&mut buffer).expect("Failed to read request");

    let request = String::from_utf8_lossy(&buffer[..]);
    println!("Received request: {}", request);

    let response = "Hello world".as_bytes();
    stream.write(response).expect("Failed to send response");
}

fn main() {
    println!("STONKS! \n");

    let listener = TcpListener::bind("127.0.0.1:7878").expect("Failed to bind TCP");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Connection established");
                std::thread::spawn(|| handle_client(stream));
            }
            Err(e) => {
                eprintln!("Failed to establish connection -> error: {}", e)
            }
        }
        // let stream = stream.unwrap();
    }

    let price_feed_socket = UdpSocket::bind("127.0.0.1:3400").expect("Failed to bind UDP");
}

// create heap

    // let buy = Order {
    //     id: 1,
    //     price: Price::new(100.05),
    //     quantity: 100,
    //     side: Side::Buy,
    // };

    // let sell = Order {
    //     id: 2,
    //     price: Price::new(99.95),
    //     quantity: 100,
    //     side: Side::Sell,
    // };

    // let mut orderbook = OrderBook::new(1);

    // println!("sell then buy");
    // orderbook.add_order(sell.clone());
    // orderbook.add_order(buy.clone());

    // println!("buy then sell");
    // orderbook.add_order(buy.clone());
    // orderbook.add_order(sell.clone());
