#![allow(dead_code)]

use crate::api::mr_cache::mr_cache_server::MrCache;
use crate::api::mr_cache::{
    Effect, HashedKeyValues, HashedKeys, Key, KeyValues, Keys, Value, Values,
};
use redis::{Commands, RedisError};
use std::sync::Arc;
use tonic::{Request, Response, Status};

use crate::api::pool::RedisPool;

pub struct MrCacheService {
    pub(crate) pool: Arc<RedisPool>,
}

#[tonic::async_trait]
impl MrCache for MrCacheService {
    async fn set(&self, request: Request<KeyValues>) -> Result<Response<Effect>, Status> {
        let start = std::time::Instant::now();

        let inner = request.into_inner();
        let keyValues: Vec<(&str, &str)> = inner
            .key_values
            .iter()
            .map(|kv| (kv.key.as_str(), kv.value.as_str()))
            .collect();
        let redis_result: Result<(), RedisError> = self.get_connection()?.mset(&keyValues);

        match redis_result {
            Ok(_) => {
                println!("Redis SET - Time elapsed: {:?}", start.elapsed());

                Ok(Response::new(Effect { effect: true }))
            }
            Err(e) => {
                eprintln!("Failed Redis command SET: {:?}", e);
                Err(Status::internal("Failed to SET to Redis DB"))
            }
        }
    }

    async fn get(&self, request: Request<Keys>) -> Result<Response<Values>, Status> {
        let start = std::time::Instant::now();

        let inner = request.into_inner();
        let keys: Vec<&str> = inner.keys.iter().map(|k| k.key.as_str()).collect();
        let redis_result: Result<Vec<Option<String>>, RedisError> =
            self.get_connection()?.mget(&keys);

        match redis_result {
            Ok(results) => {
                let values: Vec<Value> = results
                    .into_iter()
                    .filter_map(|opt| opt.map(|val| Value { value: val }))
                    .collect();

                println!("Redis GET - Time elapsed: {:?}", start.elapsed());

                Ok(Response::new(Values { values }))
            }
            Err(e) => {
                eprintln!("Failed Redis command GET: {:?}", e);
                Err(Status::internal("Failed to GET from Redis DB"))
            }
        }
    }

    async fn hset(&self, request: Request<HashedKeyValues>) -> Result<Response<Effect>, Status> {
        Ok(Response::new(Effect { effect: true }))
    }

    async fn hget(&self, request: Request<HashedKeys>) -> Result<Response<Values>, Status> {
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
    fn get_connection(&self) -> Result<r2d2::PooledConnection<redis::Client>, Status> {
        self.pool.get().map_err(|e| {
            eprintln!("Failed to get Redis connection: {:?}", e);
            Status::internal("Failed to connect to Redis DB")
        })
    }
}
