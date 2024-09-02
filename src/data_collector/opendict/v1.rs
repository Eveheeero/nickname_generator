use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct OpendictResult {
    pub(crate) total: u32,
    pub(crate) size: u32,
    pub(crate) page: u32,
    pub(crate) data: Vec<OpendictData>,
    pub(crate) datetime: time::PrimitiveDateTime,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct OpendictData {
    pub(crate) word: String,
    pub(crate) definition: String,
    pub(crate) code: u32,
    pub(crate) r#type: String,
    pub(crate) pos: String,
    pub(crate) origin: Option<String>,
}
