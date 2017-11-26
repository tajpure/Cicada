use std::env;
mod server;

fn main() {
    let port = env::args().nth(1).expect("Missing port");
    let path = env::args().nth(2).expect("Missing path");
    println!("Cicada is listening on {}, serve on '{}'", port, path);
    server::listen(port, path);
}
