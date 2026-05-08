use bb8::Pool;
use bb8_redis::RedisConnectionManager;

pub type CachePool = Pool<RedisConnectionManager>;

pub async fn init_cache(url: &str) -> CachePool {
    let manager = RedisConnectionManager::new(url)
        .expect("Failed to create Redis connection manager");
    
    let pool = Pool::builder()
        .build(manager)
        .await
        .expect("Failed to connect to Cache");

    tracing::info!("Cache connection pool initialized (Dragonfly protocol-compatible)");
    pool
}
