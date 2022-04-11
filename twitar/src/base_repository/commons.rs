use std::collections::HashMap;
use redis::Client;
use sqlx::{Pool, Postgres};

pub struct DBInsert<T: Clone>(HashMap<String, T>);

pub struct DBUpdate<T: Clone> {
    pub inserts: HashMap<String, T>,
    pub condition: Vec<HashMap<String, T>>,
}


pub struct Db {}


pub enum DbType {
    Redis(Client),
    Postgres(&Pool<Potsgres>),
}