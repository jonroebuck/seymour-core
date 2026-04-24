use crate::diagnostic::{Diagnostic, Severity};
use crate::model::journey::Journey;

const VAGUE_WORDS: &[&str] = &["should", "appropriately", "normally"];

pub fn validate(journey: &Journey) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();

    for (si, step) in journey.steps.iter().enumerate() {
        let step_path = format!("steps[{}]", si);

        if step.rules.is_empty() {
            diagnostics.push(Diagnostic {
                message: format!("Step '{}' has no rules.", step.id),
                severity: Severity::Error,
                path: format!("{}.rules", step_path),
            });
        }

        for (ri, rule) in step.rules.iter().enumerate() {
            let rule_path = format!("{}.rules[{}]", step_path, ri);

            let lower = rule.to_lowercase();
            for word in VAGUE_WORDS {
                if lower.contains(word) {
                    diagnostics.push(Diagnostic {
                        message: format!(
                            "Rule contains vague word '{}': {}",
                            word, rule
                        ),
                        severity: Severity::Warning,
                        path: rule_path.clone(),
                    });
                }
            }

            if rule.len() < 10 {
                diagnostics.push(Diagnostic {
                    message: format!("Rule is too short ({} chars): {}", rule.len(), rule),
                    severity: Severity::Warning,
                    path: rule_path,
                });
            }
        }

        if let Some(annotations) = &step.annotations {
            for (ai, annotation) in annotations.iter().enumerate() {
                if annotation.status.is_none() {
                    diagnostics.push(Diagnostic {
                        message: format!(
                            "Annotation '{}' is missing a status.",
                            annotation.kind
                        ),
                        severity: Severity::Warning,
                        path: format!("{}.annotations[{}]", step_path, ai),
                    });
                }
            }
        }
    }

    if let Some(annotations) = &journey.annotations {
        for (ai, annotation) in annotations.iter().enumerate() {
            if annotation.status.is_none() {
                diagnostics.push(Diagnostic {
                    message: format!(
                        "Annotation '{}' is missing a status.",
                        annotation.kind
                    ),
                    severity: Severity::Warning,
                    path: format!("annotations[{}]", ai),
                });
            }
        }
    }

    diagnostics
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{
        annotation::Annotation, journey::Journey, step::Step,
    };

    fn minimal_journey(rules: Vec<String>) -> Journey {
        Journey {
            id: "test".to_string(),
            title: "Test Journey".to_string(),
            expected_outcome: "Something happens.".to_string(),
            steps: vec![Step {
                id: "step-1".to_string(),
                title: "Step 1".to_string(),
                expected_outcome: "Step result.".to_string(),
                rules,
                preconditions: None,
                errors: None,
                references: None,
                comments: None,
                annotations: None,
            }],
            preconditions: None,
            related_journeys: None,
            references: None,
            comments: None,
            annotations: None,
        }
    }

    #[test]
    fn warns_on_rule_containing_should() {
        let journey = minimal_journey(vec!["Users should submit the form.".to_string()]);
        let diagnostics = validate(&journey);
        assert!(
            diagnostics.iter().any(|d| d.severity == Severity::Warning
                && d.message.contains("should")),
            "expected a warning about 'should', got: {:?}",
            diagnostics
        );
    }

    #[test]
    fn errors_on_empty_rules() {
        let journey = minimal_journey(vec![]);
        let diagnostics = validate(&journey);
        assert!(
            diagnostics.iter().any(|d| d.severity == Severity::Error),
            "expected an error for empty rules, got: {:?}",
            diagnostics
        );
    }

    #[test]
    fn warns_on_short_rule() {
        let journey = minimal_journey(vec!["Do it.".to_string()]);
        let diagnostics = validate(&journey);
        assert!(
            diagnostics.iter().any(|d| d.severity == Severity::Warning
                && d.message.contains("too short")),
            "expected a warning about short rule, got: {:?}",
            diagnostics
        );
    }

    #[test]
    fn warns_on_annotation_missing_status() {
        let mut journey = minimal_journey(vec!["A clear, testable rule.".to_string()]);
        journey.steps[0].annotations = Some(vec![Annotation {
            kind: "open-question".to_string(),
            text: "Is this correct?".to_string(),
            status: None,
        }]);
        let diagnostics = validate(&journey);
        assert!(
            diagnostics.iter().any(|d| d.severity == Severity::Warning
                && d.message.contains("missing a status")),
            "expected a warning about missing status, got: {:?}",
            diagnostics
        );
    }

    #[test]
    fn no_diagnostics_for_valid_journey() {
        let journey = minimal_journey(vec!["Applicant must provide valid identification.".to_string()]);
        let diagnostics = validate(&journey);
        assert!(diagnostics.is_empty(), "expected no diagnostics, got: {:?}", diagnostics);
    }
}
