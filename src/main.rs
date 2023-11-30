use serde::Serialize;
use tonic::transport::Server;
use tonic::{Request, Response, Status};

use rediscache::redis_cache_server::{RedisCache, RedisCacheServer};
use rediscache::{Key, Value};

mod rediscache;

#[derive(Serialize, Debug)]
struct Cache {
    key: String,
    value: String,
}

#[derive(Serialize, Debug)]
struct RedisCacheService {
    redis: &'static Cache,
}

#[tonic::async_trait]
impl RedisCache for RedisCacheService {
    async fn get(&self, request: Request<Key>) -> Result<Response<Value>, Status> {
        println!("Got a request: {:?}", request);

        Ok(Response::new(Default::default()))
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let grpc_port = "50051";

    println!("Starting server...");
    println!("gRPC listening on: http://localhost:{}", grpc_port);

    // let service = RedisCacheServer::new();
    let address = "[::1]:".to_owned() + grpc_port;

    Server::builder()
        .add_service(/* add Service here */)
        .serve(address.parse().unwrap())
        .await
        .unwrap()
}
