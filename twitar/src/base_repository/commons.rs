use std::collections::HashMap;

pub struct DBInsert<T: Clone>(HashMap<String, T>);

pub struct DBUpdate<T: Clone> {
    pub inserts: HashMap<String, T>,
    pub condition: Vec<HashMap<String, T>>,
}


pub struct Db {}