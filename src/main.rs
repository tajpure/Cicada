use std::env;
mod server;

fn main() {
    if let Some(arg1) = env::args().nth(1) {
        println!("Cicada is listening at {}", arg1);
        server::listen(arg1);
    }
}
