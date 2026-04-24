use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::annotation::Annotation;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct FormalReference {
    #[serde(rename = "type")]
    pub reference_type: String,
    pub description: String,
    pub link: Option<String>,
    pub comments: Option<Vec<String>>,
    pub annotations: Option<Vec<Annotation>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct InformalReference {
    #[serde(rename = "type")]
    pub reference_type: String,
    pub description: String,
    pub link: Option<String>,
    pub comments: Option<Vec<String>>,
    pub annotations: Option<Vec<Annotation>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct References {
    pub formal: Option<Vec<FormalReference>>,
    pub informal: Option<Vec<InformalReference>>,
}
