use std::{collections::HashMap, borrow::Cow};

pub struct ParamList(HashMap<Cow<'static, str>, Cow<'static, str>>);