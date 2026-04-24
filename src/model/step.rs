use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::annotation::Annotation;
use super::reference::References;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct Step {
    pub id: String,
    pub title: String,
    pub expected_outcome: String,
    pub rules: Vec<String>,
    pub preconditions: Option<Vec<String>>,
    pub errors: Option<Vec<String>>,
    pub references: Option<References>,
    pub comments: Option<Vec<String>>,
    pub annotations: Option<Vec<Annotation>>,
}
