use crate::config;

pub type RedisPool = r2d2::Pool<redis::Client>;

pub struct Redis;

impl Redis {
    pub fn get_pool() -> RedisPool {
        let settings = config::redis_settings();
        let database_url = settings.url;
        let client = redis::Client::open(database_url).unwrap();

        r2d2::Pool::builder()
            .max_size(settings.pool_size)
            .build(client)
            .expect("Failed to create pool")
    }
}
