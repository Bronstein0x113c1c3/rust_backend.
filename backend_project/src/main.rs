use anyhow::{Context, Result};
use sqlx::{
    mysql::{self, MySqlPool},
    pool,
    prelude::FromRow, Acquire,
};

#[derive(FromRow, Debug)]
struct user {
    id: i32,
    user_name: String,
    first_name: Option<String>,
    last_name: Option<String>,
    password: String,
    dob: Option<chrono::NaiveDate>,
}
#[ tokio::main]
async fn main() -> Result<()> {
    let options = mysql::MySqlConnectOptions::new()
        .host("localhost")
        .port(3306)
        .database("crud")
        .username("root")
        .password("0x113c1c3");
    let pool_options = mysql::MySqlPoolOptions::new().max_connections(10);
    // pool_options.max_connections(10);
    let mut pool = pool_options.connect_with(options).await?;
    // pool.begin_with(statement)s
    // mysql::MySqlPool::connect();
    let rows = sqlx::query_as::<_, user>("select * from users")
        .fetch_all(&pool)
        .await?;
    // sqlx::MySqlTransaction
    for row in rows {
        println!("{:?}", row);
    }

    Ok(())
}
