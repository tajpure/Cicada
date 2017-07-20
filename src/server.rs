use std::net::{TcpListener, TcpStream};
use std::io::Read;
use std::io::Write;
use std::thread;

pub fn listen(port: String) {
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).unwrap();

    fn handle_client(mut stream: TcpStream) {
        let mut buf;
        loop {
            buf = [0; 512];
            let _ = match stream.read(&mut buf) {
                Err(e) => panic!("error: {}", e),
                Ok(m) => {
                    if m == 0 {
                        break;
                    }
                    m
                },
            };
            println!("{}", String::from_utf8(buf.to_vec()).unwrap());
            let response = b"HTTP/1.1 200 OK
Date: Thu, 20 Jul 2017 05:41:19 GMT
Content-length: 2
Content-type: application/json

ok";

            match stream.write(response) {
                Err(_) => break,
                Ok(_) => break,
            }
        }
    }

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(e) => {
                println!("error: {}", e)
            }
        }
    }
}
