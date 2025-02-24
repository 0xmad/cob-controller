pub struct RedisSettings {
    pub url: String,
    pub pool_size: u32,
}

pub struct RabbitMQSettings {
    pub url: String,
    pub queue: String,
    pub consumer_tag: String,
}

pub fn redis_settings() -> RedisSettings {
    RedisSettings {
        url: get_env("REDIS_URL"),
        pool_size: get_env("POOL_SIZE")
            .parse::<u32>()
            .expect("POOL_SIZE shoudl be number"),
    }
}

pub fn rabbit_mq_settings() -> RabbitMQSettings {
    RabbitMQSettings {
        url: get_env("RABBIT_MQ_URL"),
        queue: get_env("QUEUE_NAME"),
        consumer_tag: get_env("CONSUMER_TAG"),
    }
}

pub fn init_envs(file: &str) {
    dotenvy::from_filename(file).ok();
}

pub fn get_env(key: &str) -> String {
    dotenvy::var(key).unwrap_or_else(|_| panic!("{} must be set", key))
}
