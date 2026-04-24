use crate::error::SeymourError;
use crate::model::journey::Journey;

#[derive(serde::Deserialize)]
struct SeymourDocument {
    journey: Journey,
}

pub fn parse(input: &str) -> Result<Journey, SeymourError> {
    let doc: SeymourDocument =
        serde_yaml::from_str(input).map_err(|e| SeymourError::ParseError(e.to_string()))?;
    Ok(doc.journey)
}
