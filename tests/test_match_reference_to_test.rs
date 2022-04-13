use grepkin::compare::json_diff_equals;
/// Feature: Matching reference feature files to grepkin tests
///   As a developer
///   I need to check that reference Gherkin features match with grepkin test comments
///   So that I know my project has the right features
use grepkin::GherkinProject;

// This feels like a quine but for grepkin: So cool!

/// Scenario: Single reference feature matches up with test code
#[test]
fn test_match_reference_to_test() {
    // Given a reference Gherkin feature in "features/"
    let reference_feature_glob = "features/match_features_test.feature".to_string();
    // And test code under "tests/"
    let test_code_glob = "tests/test_match_reference_to_test.rs".to_string();
    // When I parse the project via grepkin
    let project = GherkinProject::new(reference_feature_glob, test_code_glob);
    // Then both reference feature and grepkin test are found
    let reference = project
        .references
        .iter()
        .next()
        .expect("Error getting reference feature");
    let parsed = project
        .parsed
        .iter()
        .next()
        .expect("Error getting test code comments");
    // And reference feature matches test code
    assert!(
        json_diff_equals(parsed, reference),
        "Reference feature should match test code comments"
    );
}
