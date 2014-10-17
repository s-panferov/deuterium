
use query::{Query, RcQuery};

#[deriving(Send, Clone)]
pub struct RawQuery {
    pub content: String
}

impl Query for RawQuery { }