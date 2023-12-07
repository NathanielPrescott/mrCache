use r2d2_redis::{r2d2, RedisConnectionManager};
use std::sync::Arc;

pub type RedisPool = r2d2::Pool<RedisConnectionManager>;

pub struct Pool {
    pool: Arc<RedisPool>,
}

impl Pool {
    pub fn new() -> Self {
        let ip = "127.0.0.1"; //TODO: Extract to config
        let port = "6379"; //TODO: Extract to config
        let connection_info = "redis://".to_string() + ip + ":" + port;

        let start = std::time::Instant::now();
        let manager =
            RedisConnectionManager::new(connection_info).expect("Failed to create Redis manager");
        let pool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create Redis pool");
        println!("Redis New Pool - Time elapsed: {:?}", start.elapsed());

        Self {
            pool: Arc::new(pool),
        }
    }

    pub fn get_pool(&self) -> Arc<RedisPool> {
        let start = std::time::Instant::now();
        let cloned = self.pool.clone();
        println!("Redis Clone Pool - Time elapsed: {:?}", start.elapsed());

        cloned
    }
}
