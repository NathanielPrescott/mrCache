#![allow(dead_code)]

use crate::api::mr_cache::mr_cache_server::MrCache;
use crate::api::mr_cache::{
    Effect, Effects, HashedKeyValues, HashedValues, Key, KeyValue, KeyValues, Keys, Value, Values,
};
use r2d2_redis::r2d2::PooledConnection;
use r2d2_redis::redis::Commands;
use r2d2_redis::{redis, RedisConnectionManager};
use std::sync::Arc;
use tonic::{Request, Response, Status};

use crate::api::pool::RedisPool;

pub struct MrCacheService {
    pub(crate) pool: Arc<RedisPool>,
}

#[tonic::async_trait]
impl MrCache for MrCacheService {
    async fn set(&self, request: Request<KeyValue>) -> Result<Response<Effect>, Status> {
        let start = std::time::Instant::now();
        let kv = request.into_inner();

        let redis_result: Result<(), redis::RedisError> =
            self.get_connection()?.set(kv.key, kv.value);

        println!("Redis SET - Time elapsed: {:?}", start.elapsed());

        match redis_result {
            Ok(_) => Ok(Response::new(Effect { effect: true })),
            Err(e) => {
                eprintln!("Failed Redis command SET: {:?}", e);
                Err(Status::internal("Failed to SET to Redis DB"))
            }
        }
    }

    async fn mset(&self, request: Request<KeyValues>) -> Result<Response<Effects>, Status> {
        println!("Set: {:?}", request.into_inner());

        // This should implement MSET

        Ok(Response::new(Effects {
            effects: vec![Effect { effect: true }, Effect { effect: true }],
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

        Ok(Response::new(Effect { effect: true }))
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

impl MrCacheService {
    fn get_connection(&self) -> Result<PooledConnection<RedisConnectionManager>, Status> {
        self.pool.get().map_err(|e| {
            eprintln!("Failed to get Redis connection: {:?}", e);
            Status::internal("Failed to connect to Redis DB")
        })
    }
}
