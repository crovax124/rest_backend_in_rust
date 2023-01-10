#![allow(dead_code)]
mod server;
mod http;
mod website_handler;


use std::env;
use server::Server;
use website_handler::WebsiteHandler;

fn main() {
   // let get = http::Method::GET;
   // let post = http::Method::POST;
   // let put = http::Method::PUT;
   // let delete = http::Method::DELETE;

    //holt sich den path der directory auf dem pc auf dem es läuft.
    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    //kann mit eingabe gestartet werden damit eine eigene variable übernommen werden also server mit
    // PUBLIC_PATH=$(pwd)/public cargo run
    // starten
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);

    println!("Serving files from: {}", public_path);

    let server = Server::new("127.0.0.1:8080".to_string());
    server.run(WebsiteHandler::new(public_path));
}