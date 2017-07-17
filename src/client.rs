use std::net::TcpStream;

pub fn connect(host: String, port: String) {
    if let Ok(stream) = TcpStream::connect(format!("{}:{}", host , port)) {
        println!("Connected to the server!");
    } else {
        println!("Couldn't connect to server...");
    }
}
