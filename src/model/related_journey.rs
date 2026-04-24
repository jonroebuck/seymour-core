use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct RelatedJourney {
    pub id: String,
    pub relationship: String,
    pub required: bool,
    pub condition: Option<String>,
}
