#![allow(non_snake_case)]

use crate::api::client::MrCacheService;
use crate::api::mr_cache::mr_cache_server::MrCacheServer;
use crate::api::pool::Pool;
use tonic::transport::Server;

mod api {
    pub mod client;
    pub mod mr_cache;
    pub mod pool;
}

#[tokio::main]
async fn main() {
    let grpc_port = "50051";
    let pool = Pool::new();

    println!("Starting server...");
    println!("gRPC listening on: http://localhost:{}", grpc_port);

    let address = ("0.0.0.0:".to_owned() + grpc_port).parse().unwrap();

    Server::builder()
        .add_service(MrCacheServer::new(MrCacheService {
            pool: pool.get_pool(),
        }))
        .serve(address)
        .await
        .expect("Server failed to start!")
}
