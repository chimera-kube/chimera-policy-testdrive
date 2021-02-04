use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct ValidationResponse {
    pub accepted: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<u16>,
}
