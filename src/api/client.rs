#![allow(dead_code)]

use crate::api::mr_cache::mr_cache_server::MrCache;
use crate::api::mr_cache::{
    Effect, HashedKeyValues, HashedKeys, Key, KeyValues, Keys, Value, Values,
};
use r2d2::PooledConnection;
use redis::{Client, Commands, RedisResult};
use std::sync::Arc;
use tonic::{Request, Response, Status};

use crate::api::pool::RedisPool;

pub struct MrCacheService {
    pub(crate) pool: Arc<RedisPool>,
}

#[tonic::async_trait]
impl MrCache for MrCacheService {
    async fn set(&self, request: Request<KeyValues>) -> Result<Response<Effect>, Status> {
        let inner = request.into_inner();
        let keyValues: Vec<(&str, &str)> = inner
            .key_values
            .iter()
            .map(|kv| (kv.key.as_str(), kv.value.as_str()))
            .collect();

        self.execute_redis_cmd(
            "SET",
            |mut con| con.mset(&keyValues),
            |_: ()| Effect { effect: true },
        )
        .await
    }

    async fn get(&self, request: Request<Keys>) -> Result<Response<Values>, Status> {
        let inner = request.into_inner();
        let keys: Vec<&str> = inner.keys.iter().map(|k| k.key.as_str()).collect();

        self.execute_redis_cmd(
            "GET",
            |mut con| con.mget(keys),
            |results: Vec<Option<String>>| {
                let values: Vec<Value> = results
                    .into_iter()
                    .filter_map(|opt| opt.map(|val| Value { value: val }))
                    .collect();
                Values { values }
            },
        )
        .await
    }

    async fn hset(&self, request: Request<HashedKeyValues>) -> Result<Response<Effect>, Status> {
        let inner = request.into_inner();
        let key = inner.key.unwrap().key;
        let keyValues = inner.key_values.unwrap().key_values;
        let fieldValues: Vec<(&str, &str)> = keyValues
            .iter()
            .map(|kv| (kv.key.as_str(), kv.value.as_str()))
            .collect();

        self.execute_redis_cmd(
            "HSET",
            |mut con| con.hset_multiple(key, &fieldValues),
            |_: ()| Effect { effect: true },
        )
        .await
    }

    async fn hget(&self, request: Request<HashedKeys>) -> Result<Response<Values>, Status> {
        let inner = request.into_inner();
        let key = inner.key.unwrap().key;
        let keys = inner.keys.unwrap().keys;
        let fields: Vec<&str> = keys.iter().map(|k| k.key.as_str()).collect();

        self.execute_redis_cmd(
            "HGET",
            |mut con| con.hget(key, &fields),
            |results: Vec<Option<String>>| {
                let values: Vec<Value> = results
                    .into_iter()
                    .filter_map(|opt| opt.map(|val| Value { value: val }))
                    .collect();
                Values { values }
            },
        )
        .await
    }

    async fn hgetall(&self, request: Request<Key>) -> Result<Response<Values>, Status> {
        let inner = request.into_inner();
        let key = inner.key;

        self.execute_redis_cmd(
            "HGETALL",
            |mut con| con.hgetall(key),
            |results: Vec<Option<String>>| {
                let values: Vec<Value> = results
                    .into_iter()
                    .filter_map(|opt| opt.map(|val| Value { value: val }))
                    .collect();
                Values { values }
            },
        )
        .await
    }

    async fn hkeys(&self, request: Request<Key>) -> Result<Response<Keys>, Status> {
        let inner = request.into_inner();
        let key = inner.key;

        self.execute_redis_cmd(
            "HKEYS",
            |mut con| con.hkeys(key),
            |results: Vec<Option<String>>| {
                let keys: Vec<Key> = results
                    .into_iter()
                    .filter_map(|opt| opt.map(|k| Key { key: k }))
                    .collect();
                Keys { keys }
            },
        )
        .await
    }

    async fn hvals(&self, request: Request<Key>) -> Result<Response<Values>, Status> {
        let inner = request.into_inner();
        let key = inner.key;

        self.execute_redis_cmd(
            "HVALS",
            |mut con| con.hvals(key),
            |results: Vec<Option<String>>| {
                let values: Vec<Value> = results
                    .into_iter()
                    .filter_map(|opt| opt.map(|val| Value { value: val }))
                    .collect();
                Values { values }
            },
        )
        .await
    }
}

impl MrCacheService {
    fn get_connection(&self) -> Result<PooledConnection<Client>, Status> {
        self.pool.get().map_err(|e| {
            eprintln!("Failed to get Redis connection: {:?}", e);
            Status::internal("Failed to connect to Redis DB")
        })
    }

    async fn execute_redis_cmd<T, F, G, R>(
        &self,
        cmd: &str,
        redis_cmd: F,
        transform: G,
    ) -> Result<Response<R>, Status>
    where
        F: FnOnce(PooledConnection<Client>) -> RedisResult<T>,
        G: FnOnce(T) -> R,
    {
        let start = std::time::Instant::now();

        match redis_cmd(self.get_connection()?) {
            Ok(results) => {
                let transformed = transform(results);
                println!("Redis {} - Time elapsed: {:?}", cmd, start.elapsed());

                Ok(Response::new(transformed))
            }
            Err(e) => Err({
                eprintln!("Failed Redis command {}: {:?}", cmd, e);
                Status::internal("Failed to use ".to_string() + cmd + " from Redis DB")
            }),
        }
    }
}
