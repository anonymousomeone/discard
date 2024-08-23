use serde::{Serialize, Deserialize};

#[derive(Deserialize, Debug)]
pub struct HELLO_10 {
    pub op: u16,
    pub s: Option<u32>,
    pub t: Option<String>,
    pub d: Option<String>
}
