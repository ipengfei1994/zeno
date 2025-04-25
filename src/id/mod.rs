pub mod luhmann;
pub mod timestamp;
pub mod uuid;

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ZenoId {
    Timestamp(String),
    Uuid(String),
    Luhmann(String),
}

impl fmt::Display for ZenoId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ZenoId::Timestamp(id) => write!(f, "{}", id),
            ZenoId::Uuid(id) => write!(f, "{}", id),
            ZenoId::Luhmann(id) => write!(f, "{}", id),
        }
    }
}
