use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::annotation::Annotation;
use super::reference::References;
use super::related_journey::RelatedJourney;
use super::step::Step;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct Journey {
    pub id: String,
    pub title: String,
    pub expected_outcome: String,
    pub steps: Vec<Step>,
    pub preconditions: Option<Vec<String>>,
    pub related_journeys: Option<Vec<RelatedJourney>>,
    pub references: Option<References>,
    pub comments: Option<Vec<String>>,
    pub annotations: Option<Vec<Annotation>>,
}
