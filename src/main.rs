use std::env;
mod server;

fn main() {
    if let Some(port) = env::args().nth(1) {
        println!("Cicada is listening on {}", port);
        server::listen(port);
    }
}
