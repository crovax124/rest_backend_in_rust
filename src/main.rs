#![allow(dead_code)]
mod server;
mod http;
mod website_handler;


use server::Server;
use http::Request;
use website_handler::WebsiteHandler;

fn main() {
   // let get = http::Method::GET;
   // let post = http::Method::POST;
   // let put = http::Method::PUT;
   // let delete = http::Method::DELETE;
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run(WebsiteHandler);
}