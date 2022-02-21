use std::{collections::HashMap, borrow::Cow};

#[derive(Debug, Clone, Default, derive_more::Deref, derive_more::DerefMut, derive_more::From)]
pub struct RequestParams(HashMap<Cow<'static, str>, Cow<'static, str>>);



impl RequestParams {
    pub fn new() -> Self {
        RequestParams(HashMap::new())
    }

    pub fn add_param(mut self, 
        key: impl Into<Cow<'static, str>>, 
        value: impl Into<Cow<'static, str>>) -> Self 
    {
        self.insert(key.into(), value.into());
        self
    }

    pub fn add_opt_param(self, 
        key: impl Into<Cow<'static, str>>,
        value: Option<impl Into<Cow<'static, str>>>
    ) -> Self {
        match value {
            Some(v) => self.add_param(key.into(), v.into()),
            _ => self
        }
    }
    
}