use std::net::{TcpListener, TcpStream};
use std::io::Read;
use std::io::Write;
use std::thread;
use std::fs::File;

pub fn listen(port: String, path: String) {
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).unwrap();

    fn handle_client(mut stream: TcpStream, path: String) {
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

            let mut file = File::open(path).unwrap();
            let mut buffer = String::new();

            file.read_to_string(&mut buffer);

            let response = format!("HTTP/1.1 200 OK\n \
                            Date: Thu, 20 Jul 2017 05:41:19 GMT\n \
                            Content-length: 12\n \
                            Content-type: text/html\n\n{}", buffer);

            match stream.write(response.as_bytes()) {
                Err(_) => break,
                Ok(_) => break,
            }
        }
    }

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let path = path.clone();
                thread::spawn(move || {
                    handle_client(stream, path);
                });
            }
            Err(e) => {
                println!("error: {}", e)
            }
        }
    }
}
