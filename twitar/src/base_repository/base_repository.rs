use futures::{stream, StreamExt};
use redis::{AsyncCommands, ToRedisArgs, RedisError};
use std::{fmt::Display};

use crate::{startup::server::{AppState, LocalAppState}, helpers::commons::AppEnv, configurations::variables::SettingsVars};

pub type ColsAndVals<V: Display + Send + Sync + ToRedisArgs> = Vec<(&'static str, V)>;


pub struct DBInsert<V: Display + Send + Sync + ToRedisArgs> {
    table: String,
    cols_vals: ColsAndVals<V>,
    returning: bool,
}


impl<V: Display + Send + Sync + ToRedisArgs> DBInsert<V> {
    pub fn create(table: String, cols_vals: ColsAndVals<V>, returning: bool) -> Self {
        // println!("")
        Self {table, cols_vals, returning}
    }

    pub async fn execute(self, app_state: LocalAppState) {
        let LocalAppState { redis , app_env, db_pool, .. } = app_state;

        println!("HERE NOW IN THE EXECUTE!");
        
        match app_env {
            AppEnv::Local => {
                let mut con = redis.get_async_connection().await.unwrap();

                for (key, val) in &self.cols_vals {
                    // this isn't the appropriate return type in this use case but ok
                    let _d: String = con.set(key, val).await.unwrap();
                }
            },
            AppEnv::Test | AppEnv::Staging | AppEnv::Production => {
                let mut col_names: Vec<&'static str> = vec![];
                let mut positions: Vec<String> = vec![];
                let mut vals: Vec<&V> = vec![];

                for (index, (key, val)) in self.cols_vals.iter().enumerate() {
                    col_names.push(key);
                    positions.push(format!("${}", index));
                    vals.push(val);
                }                

                let querry_str = format!(
                    r#"INSERT INTO {} ({}) VALUES ({})"#,
                    self.table, 
                    col_names.join(", "),
                    positions.join(", "),
                );

                // sqlx::query!(querry_str).execute(&db_pool).await.unwrap();
            },
        }
    }
}