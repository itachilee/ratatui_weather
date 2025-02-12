use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenv::dotenv;
use once_cell::sync::Lazy;
use std::env;

// 定义连接池类型
pub type PgPool = Pool<ConnectionManager<PgConnection>>;

// 全局连接池实例
pub static POOL: Lazy<PgPool> = Lazy::new(|| {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
});
