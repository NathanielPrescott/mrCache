#![allow(non_snake_case)]
use tonic::transport::Server;

mod api {
    pub mod client;
    pub mod mr_cache;
}

#[tokio::main]
async fn main() {
    let grpc_port = "50051";

    println!("Starting server...");
    println!("gRPC listening on: http://localhost:{}", grpc_port);

    let address = ("[::1]:".to_owned() + grpc_port).parse().unwrap();

    Server::builder()
        .add_service(api::client::init_mr_cache_server())
        .serve(address)
        .await
        .unwrap()
}
