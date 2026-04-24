use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct Annotation {
    pub kind: String,
    pub text: String,
    pub status: Option<String>,
}
