use crate::api::mr_cache::mr_cache_server::{MrCache, MrCacheServer};
use crate::api::mr_cache::{
    Effect, Effects, HashedKeyValues, HashedValues, Key, KeyValue, KeyValues, Keys, Value, Values,
};

use tonic::{Request, Response, Status};

use serde::Serialize;

#[derive(Serialize, Debug)]
struct Cache {
    key: String,
    value: String,
}

#[derive(Serialize, Debug)]
pub struct MrCacheService {
    cache: &'static Cache,
}

#[tonic::async_trait]
impl MrCache for MrCacheService {
    async fn set(&self, request: Request<KeyValue>) -> Result<Response<Effect>, Status> {
        println!("Set: {:?}", request.into_inner());

        // This should implement SET

        Ok(Response::new(Effect {
            effect: "SET gRPC call temp effect".to_string(),
        }))
    }

    async fn mset(&self, request: Request<KeyValues>) -> Result<Response<Effects>, Status> {
        println!("Set: {:?}", request.into_inner());

        // This should implement MSET

        Ok(Response::new(Effects {
            effects: vec![
                Effect {
                    effect: "MSET gRPC call temp effect 1".to_string(),
                },
                Effect {
                    effect: "MSET gRPC call temp effect 2".to_string(),
                },
            ],
        }))
    }

    async fn get(&self, request: Request<Key>) -> Result<Response<Value>, Status> {
        println!("Get: {:?}", request.into_inner());

        // This should implement GET

        Ok(Response::new(Value {
            value: "GET gRPC call temp value".to_string(),
        }))
    }

    async fn mget(&self, request: Request<Keys>) -> Result<Response<Values>, Status> {
        println!("Get: {:?}", request.into_inner());

        // This should implement MGET

        Ok(Response::new(Values {
            values: vec![
                Value {
                    value: "MGET gRPC call temp value 1".to_string(),
                },
                Value {
                    value: "MGET gRPC call temp value 2".to_string(),
                },
            ],
        }))
    }

    async fn hset(&self, request: Request<HashedKeyValues>) -> Result<Response<Effect>, Status> {
        println!("Hash Set: {:?}", request.into_inner());

        // This should implement HSET

        Ok(Response::new(Effect {
            effect: "HSET gRPC call temp effect".to_string(),
        }))
    }

    async fn hmget(&self, request: Request<HashedValues>) -> Result<Response<Values>, Status> {
        println!("Hash Get: {:?}", request.into_inner());

        // This should implement HMGET

        Ok(Response::new(Values {
            values: vec![
                Value {
                    value: "HMGET gRPC call temp value 1".to_string(),
                },
                Value {
                    value: "HMGET gRPC call temp value 2".to_string(),
                },
            ],
        }))
    }

    async fn hgetall(&self, request: Request<Key>) -> Result<Response<Values>, Status> {
        println!("Hash Get All: {:?}", request.into_inner());

        // This should implement HGETALL

        Ok(Response::new(Values {
            values: vec![
                Value {
                    value: "HGETALL gRPC call temp value 1".to_string(),
                },
                Value {
                    value: "HGETALL gRPC call temp value 2".to_string(),
                },
            ],
        }))
    }

    async fn hkeys(&self, request: Request<Key>) -> Result<Response<Keys>, Status> {
        println!("Hash Keys: {:?}", request.into_inner());

        // This should implement HKEYS

        Ok(Response::new(Keys {
            keys: vec![
                Key {
                    key: "HKEYS gRPC call temp key 1".to_string(),
                },
                Key {
                    key: "HKEYS gRPC call temp key 2".to_string(),
                },
            ],
        }))
    }

    async fn hvals(&self, request: Request<Key>) -> Result<Response<Values>, Status> {
        println!("Hash Values: {:?}", request.into_inner());

        // This should implement HVALS

        Ok(Response::new(Values {
            values: vec![
                Value {
                    value: "HVALS gRPC call temp value 1".to_string(),
                },
                Value {
                    value: "HVALS gRPC call temp value 2".to_string(),
                },
            ],
        }))
    }
}

pub fn init_mr_cache_server() -> MrCacheServer<MrCacheService> {
    MrCacheServer::new(MrCacheService {
        cache: Box::leak(Box::new(Cache {
            key: "key".to_string(),
            value: "value".to_string(),
        })),
    })
}
