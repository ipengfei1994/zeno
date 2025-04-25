use crate::id::ZenoId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Link {
    pub from: ZenoId,
    pub to: ZenoId,
    pub description: Option<String>,
}
