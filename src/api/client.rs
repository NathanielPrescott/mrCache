#![allow(dead_code)]

use crate::api::mr_cache::mr_cache_server::MrCache;
use crate::api::mr_cache::{
    Effect, HashedKeyValues, HashedKeys, Key, KeyValues, Keys, Value, Values,
};
use r2d2::PooledConnection;
use redis::{Client, Cmd, Commands, RedisError, RedisResult};
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

        self.set_redis_cmd("SET", |mut con| con.mset(&keyValues))
            .await
    }

    async fn get(&self, request: Request<Keys>) -> Result<Response<Values>, Status> {
        let inner = request.into_inner();
        let keys: Vec<&str> = inner.keys.iter().map(|k| k.key.as_str()).collect();

        self.retrieve_redis_cmd(
            "GET",
            |mut con| con.mget(keys),
            |results| {
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

        self.set_redis_cmd("HSET", |mut con| con.hset_multiple(key, &fieldValues))
            .await
    }

    async fn hget(&self, request: Request<HashedKeys>) -> Result<Response<Values>, Status> {
        let inner = request.into_inner();
        let key = inner.key.unwrap().key;
        let keys = inner.keys.unwrap().keys;
        let fields: Vec<&str> = keys.iter().map(|k| k.key.as_str()).collect();

        self.retrieve_redis_cmd(
            "HGET",
            |mut con| con.hget(key, &fields),
            |results| {
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

        self.retrieve_redis_cmd(
            "HGETALL",
            |mut con| con.hgetall(key),
            |results| {
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

        self.retrieve_redis_cmd(
            "HKEYS",
            |mut con| con.hkeys(key),
            |results| {
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

        self.retrieve_redis_cmd(
            "HVALS",
            |mut con| con.hvals(key),
            |results| {
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

    fn handle_redis_error(&self, e: RedisError, cmd: &str) -> Status {
        eprintln!("Failed Redis command {}: {:?}", cmd, e);
        Status::internal("Failed to use ".to_string() + cmd + " from Redis DB")
    }

    async fn retrieve_redis_cmd<T, F, G>(
        &self,
        cmd: &str,
        redis_cmd: F,
        transform: G,
    ) -> Result<Response<T>, Status>
    where
        F: FnOnce(PooledConnection<Client>) -> RedisResult<Vec<Option<String>>>,
        G: FnOnce(Vec<Option<String>>) -> T,
    {
        let start = std::time::Instant::now();
        let redis_result = redis_cmd(self.get_connection()?);

        match redis_result {
            Ok(results) => {
                let transformed = transform(results);
                println!("Redis {} - Time elapsed: {:?}", cmd, start.elapsed());

                Ok(Response::new(transformed))
            }
            Err(e) => Err(self.handle_redis_error(e, cmd)),
        }
    }

    async fn set_redis_cmd<F>(&self, cmd: &str, redis_cmd: F) -> Result<Response<Effect>, Status>
    where
        F: FnOnce(PooledConnection<Client>) -> RedisResult<()>,
    {
        let start = std::time::Instant::now();
        let redis_result = redis_cmd(self.get_connection()?);

        match redis_result {
            Ok(_) => {
                println!("Redis {} - Time elapsed: {:?}", cmd, start.elapsed());

                Ok(Response::new(Effect { effect: true }))
            }
            Err(e) => Err(self.handle_redis_error(e, cmd)),
        }
    }
}
