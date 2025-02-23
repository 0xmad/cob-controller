pub struct RedisSettings {
    pub url: String,
    pub pool_size: u32,
}

pub fn redis_settings() -> RedisSettings {
    RedisSettings {
        url: get_env("REDIS_URL"),
        pool_size: get_env("POOL_SIZE")
            .parse::<u32>()
            .expect("POOL_SIZE shoudl be number"),
    }
}

pub fn init_envs(file: &str) {
    dotenvy::from_filename(file).ok();
}

fn get_env(key: &str) -> String {
    dotenvy::var(key).unwrap_or_else(|_| panic!("{} must be set", key))
}
