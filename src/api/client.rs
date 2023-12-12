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
        let start = std::time::Instant::now();

        let inner = request.into_inner();
        let key = inner.key.unwrap().key;
        let keyValues = inner.key_values.unwrap().key_values;
        let fieldValues: Vec<(&str, &str)> = keyValues
            .iter()
            .map(|kv| (kv.key.as_str(), kv.value.as_str()))
            .collect();

        let redis_result: Result<(), RedisError> =
            self.get_connection()?.hset_multiple(key, &fieldValues);

        match redis_result {
            Ok(_) => {
                println!("Redis HSET - Time elapsed: {:?}", start.elapsed());

                Ok(Response::new(Effect { effect: true }))
            }
            Err(e) => {
                eprintln!("Failed Redis command HSET: {:?}", e);
                Err(Status::internal("Failed to HSET to Redis DB"))
            }
        }
    }

    async fn hget(&self, request: Request<HashedKeys>) -> Result<Response<Values>, Status> {
        let start = std::time::Instant::now();

        let inner = request.into_inner();
        let key = inner.key.unwrap().key;
        let keys = inner.keys.unwrap().keys;
        let fields: Vec<&str> = keys.iter().map(|k| k.key.as_str()).collect();

        let redis_result: Result<Vec<Option<String>>, RedisError> =
            self.get_connection()?.hget(key, &fields);

        match redis_result {
            Ok(results) => {
                let values: Vec<Value> = results
                    .into_iter()
                    .filter_map(|opt| opt.map(|val| Value { value: val }))
                    .collect();

                println!("Redis HGET - Time elapsed: {:?}", start.elapsed());

                Ok(Response::new(Values { values }))
            }
            Err(e) => {
                eprintln!("Failed Redis command HGET: {:?}", e);
                Err(Status::internal("Failed to HGET from Redis DB"))
            }
        }
    }

    async fn hgetall(&self, request: Request<Key>) -> Result<Response<Values>, Status> {
        let start = std::time::Instant::now();

        let inner = request.into_inner();
        let key = inner.key;

        let redis_result: Result<Vec<Option<String>>, RedisError> =
            self.get_connection()?.hgetall(key);

        match redis_result {
            Ok(results) => {
                let values: Vec<Value> = results
                    .into_iter()
                    .filter_map(|opt| opt.map(|val| Value { value: val }))
                    .collect();

                println!("Redis HGETALL - Time elapsed: {:?}", start.elapsed());

                Ok(Response::new(Values { values }))
            }
            Err(e) => {
                eprintln!("Failed Redis command HGETALL: {:?}", e);
                Err(Status::internal("Failed to HGETALL from Redis DB"))
            }
        }
    }

    async fn hkeys(&self, request: Request<Key>) -> Result<Response<Keys>, Status> {
        let start = std::time::Instant::now();

        let inner = request.into_inner();
        let key = inner.key;

        let redis_result: Result<Vec<Option<String>>, RedisError> =
            self.get_connection()?.hkeys(key);

        match redis_result {
            Ok(results) => {
                let keys: Vec<Key> = results
                    .into_iter()
                    .filter_map(|opt| opt.map(|k| Key { key: k }))
                    .collect();

                println!("Redis HKEYS - Time elapsed: {:?}", start.elapsed());

                Ok(Response::new(Keys { keys }))
            }
            Err(e) => {
                eprintln!("Failed Redis command HKEYS: {:?}", e);
                Err(Status::internal("Failed to HKEYS from Redis DB"))
            }
        }
    }

    async fn hvals(&self, request: Request<Key>) -> Result<Response<Values>, Status> {
        let start = std::time::Instant::now();

        let inner = request.into_inner();
        let key = inner.key;

        let redis_result: Result<Vec<Option<String>>, RedisError> =
            self.get_connection()?.hvals(key);

        match redis_result {
            Ok(results) => {
                let values: Vec<Value> = results
                    .into_iter()
                    .filter_map(|opt| opt.map(|val| Value { value: val }))
                    .collect();

                println!("Redis HVALS - Time elapsed: {:?}", start.elapsed());

                Ok(Response::new(Values { values }))
            }
            Err(e) => {
                eprintln!("Failed Redis command HVALS: {:?}", e);
                Err(Status::internal("Failed to HVALS from Redis DB"))
            }
        }
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
