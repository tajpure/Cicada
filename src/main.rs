use std::env;
mod client;
mod server;

fn main() {
    if let Some(arg1) = env::args().nth(1) {
        match arg1.as_ref() {
            "server" => {
                let arg2 = env::args().nth(2);
                server::listen(arg2.unwrap())
            },
            "client" => {
                let arg2 = env::args().nth(2);
                let arg3 = env::args().nth(3);
                client::connect(arg2.unwrap(), arg3.unwrap())
            },
            _ => println!("not support yet!"),
        }
    }
}
