use crate::redis::RedisPool;

pub struct Context {
    pub redis_pool: RedisPool,
}
