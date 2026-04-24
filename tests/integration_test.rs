use seymour_core::parser::parse;

#[test]
fn parses_cdcp_enrollment_fixture() {
    let input = include_str!("fixtures/cdcp-enrollment.yaml");
    let result = parse(input);
    assert!(result.is_ok(), "expected Ok, got: {:?}", result);
    let journey = result.unwrap();
    assert_eq!(journey.id, "cdcp-enrollment");
}
