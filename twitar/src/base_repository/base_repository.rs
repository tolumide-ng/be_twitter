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

        let querry = String::from("");
        println!("HERE NOW IN THE EXECUTE!");
        
        match app_env {
            AppEnv::Local => {
                println!("BAD MAN ON THE LOCAL ENV");
                let mut con = redis.get_async_connection().await.unwrap();

                for (key, val) in &self.cols_vals {
                    // this isn't the appropriate return type in this use case but ok
                    let d: String = con.set(key, val).await.unwrap();
                    println!("THIUS SHOULD WORK {:#?} {}", key, val);
                }
            },
            AppEnv::Test | AppEnv::Staging | AppEnv::Production => {},
        }
    }
}