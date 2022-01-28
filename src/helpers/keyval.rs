use std::{collections::HashMap, borrow::Cow};
use derive_more;

#[derive(Debug, derive_more::Deref, derive_more::DerefMut, derive_more::From, Clone, Default)]
pub struct KeyVal(HashMap<Cow<'static, str>, Cow<'static, str>>);


impl KeyVal {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn add_keyval(self, key: Cow<'static, str>, val: Cow<'static, str>) -> Self {
        self.insert(key, val);
        self
    }

    pub fn to_urlencode(&self) -> String {
        let ab = self.iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<_>>().join("&");
        String::from("")
    }
}