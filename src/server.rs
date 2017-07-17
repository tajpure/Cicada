use std::net::{TcpListener, TcpStream};
use std::io::Read;
use std::io::Write;

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
            match stream.write(&buf) {
                Err(_) => break,
                Ok(_) => continue,
            }
        }
    }

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream);
            }
            Err(e) => {
                println!("error: {}", e)
            }
        }
    }
}
