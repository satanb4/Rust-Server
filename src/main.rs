use http::Request;
use http::Method;
use server::Server;

mod http;
mod server;

fn main() {
    /*
    Possible enum methods
    let get = Method::GET("abcd".to_string());
    let delete = Method::DELETE(100);
    let post = Method::POST;
    */

    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}

