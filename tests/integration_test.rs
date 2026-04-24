use seymour_core::parser::parse;
use seymour_core::validator::validate;

#[test]
fn parses_cdcp_enrollment_fixture() {
    let input = include_str!("fixtures/cdcp-enrollment.yaml");
    let result = parse(input);
    assert!(result.is_ok(), "expected Ok, got: {:?}", result);
    let journey = result.unwrap();
    assert_eq!(journey.id, "cdcp-enrollment");
}

#[test]
fn validates_cdcp_enrollment_fixture_with_no_diagnostics() {
    let input = include_str!("fixtures/cdcp-enrollment.yaml");
    let journey = parse(input).expect("fixture should parse");
    let diagnostics = validate(&journey);
    assert!(
        diagnostics.is_empty(),
        "expected no diagnostics for valid fixture, got: {:?}",
        diagnostics
    );
}
