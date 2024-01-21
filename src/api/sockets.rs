use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream, UdpSocket},
};

fn handle_tcp_client(mut stream: TcpStream) -> std::io::Result<()> {
    let mut buffer: [u8; 1024] = [0; 1024];

    stream.read(&mut buffer).expect("Failed to read request");

    let request = String::from_utf8_lossy(&buffer[..]);
    println!("Received request: {}", request);

    match request.trim() {
        "buy" => {

            let response = "Sell order received";
            stream.write(response.as_bytes())?;
        },
        "sell" => {
            
            let response = "Sell order received";
            stream.write(response.as_bytes())?;
        },
        _ => {
            let response = "Invalid order type";
            stream.write(response.as_bytes())?;
        },
    };
    Ok(())
}

// fn main() {
//     println!("STONKS! \n");

//     let listener = TcpListener::bind("127.0.0.1:7878").expect("Failed to bind TCP");

//     for stream in listener.incoming() {
//         match stream {
//             Ok(stream) => {
//                 println!("Connection established");
//                 std::thread::spawn(|| handle_tcp_client(stream));
//             }
//             Err(e) => {
//                 eprintln!("Failed to establish connection -> error: {}", e)
//             }
//         }
//         // let stream = stream.unwrap();
//     }

//     // let price_feed_socket = UdpSocket::bind("127.0.0.1:3400").expect("Failed to bind UDP");
// }
