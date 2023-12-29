use std::sync::Arc;
use std::time::Duration;

pub type RedisPool = r2d2::Pool<redis::Client>;

pub struct Pool {
    pool: Arc<RedisPool>,
}

impl Pool {
    pub fn new() -> Self {
        let ip = "host.docker.internal"; //TODO: Extract to config
        let port = "6379"; //TODO: Extract to config
        let connection_info = "redis://".to_string() + ip + ":" + port;

        let start = std::time::Instant::now();

        let client: redis::Client =
            redis::Client::open(connection_info).expect("Failed to open connection manager.");
        let pool: r2d2::Pool<redis::Client> = r2d2::Pool::builder()
            .connection_timeout(Duration::from_millis(200))
            .build(client)
            .expect("Failed to create/connect Redis pool.");

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
