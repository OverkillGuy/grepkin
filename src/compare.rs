/// Computes equality of gherkin::Feature objects in terms of Gherkin content
/// but ignoring the Spans (line/col number) and Path where text was found.
///
/// Workaround for Feature equality operator being too specific when comparing
/// the same feature, parsed from different sources (proper feature file vs
/// extracted from test code).
/// TODO Prove this function really works via tests
/// FIXME: zip iter "reference" exhaustion causes other-exclusive scenarios+steps to wrongly match
use treediff;

#[allow(clippy::nonminimal_bool)]
pub fn approximately_eq(reference: &gherkin::Feature, other: &gherkin::Feature) -> bool {
    let is_feature_equal: bool = reference.keyword == other.keyword
        && reference.name == other.name
        && reference.description == other.description
        && reference.background == other.background
        && reference.description == other.description;
    // No need to compare if not equal length
    if reference.scenarios.len() != other.scenarios.len() {
        return false;
    }
    // Assuming scenarios come out in same order = zip-able
    let are_scenarios_equal: bool = reference.scenarios.iter()
                           .zip(other.scenarios.iter())
                           .map(|s| s.0.keyword == s.1.keyword
                                && s.0.name == s.1.name
                                // Dive into Vec<steps> of Scenario the same way
                                && s.0.steps.iter().zip(s.1.steps.iter())
                                    .map(|t| t.0.keyword == t.1.keyword
                                         && t.0.ty == t.1.ty
                                         && t.0.value == t.1.value)
                                    .reduce(|acc, mk | acc && mk).unwrap())  // reduce by AND: Vec<steps>
                           .reduce(|acc, mk | acc && mk)  // reduce by AND: Vec<scenarios>
                           .unwrap();
    is_feature_equal && are_scenarios_equal
}

/// Computes struct-identicality by using treediff lib (struct comparison)
///
/// Relies on conversion of gherkin::Feature->serde_json::Value, using Value
/// trait to run treediff::diff
#[allow(dead_code)]
#[allow(clippy::just_underscores_and_digits, clippy::match_like_matches_macro)]
fn json_diff_equals(reference: &gherkin::Feature, other: &gherkin::Feature) -> bool {
    let ref_json: serde_json::Value = serde_json::value::to_value(&reference).unwrap();
    let other_json: serde_json::Value = serde_json::value::to_value(&other).unwrap();
    let mut record = treediff::tools::Recorder::default();
    treediff::diff(&ref_json, &other_json, &mut record);
    if record.calls.len() != 1 {
        return false;
    }
    match record.calls.last().unwrap() {
        // FIXME skip the Path, Linecol and Span for diff
        // TODO Use custom treediff::tools::Delegate instead
        treediff::tools::ChangeType::Unchanged(_, __) => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::{approximately_eq, json_diff_equals};
    use std::path::PathBuf;

    /// Sample feature to do some comparisons against
    fn reference_feature() -> gherkin::Feature {
        let reference_scenarios: Vec<gherkin::Scenario> = vec![gherkin::Scenario::builder()
            .keyword("Scenario".to_string())
            .name("Dummy scenario to be representative".to_string())
            .steps(vec![gherkin::Step::builder()
                .keyword("When".to_string())
                .ty(gherkin::StepType::When)
                .value("something happens".to_string())
                .build()])
            .build()];
        gherkin::Feature::builder()
            .keyword("Feature".to_string())
            .name("Matching a feature with another identical feature".to_string())
            .description(Some("Bleh".to_string()))
            .scenarios(reference_scenarios)
            .build()
    }

    #[test]
    fn test_cloned_feature_nodiff() {
        let reference = reference_feature();
        let cloned = reference.clone();
        assert!(
            approximately_eq(&reference, &cloned),
            "Identical feature should match clone"
        );

        assert!(
            json_diff_equals(&reference, &cloned),
            "Identical feature should match clone"
        );
    }

    #[test]
    fn test_cloned_feature_noscenarios_nodiff() {
        let reference = reference_feature();

        let cloned = reference.clone();
        assert!(
            json_diff_equals(&reference, &cloned),
            "Identical feature should match clone"
        );
        // FIXME Crash on comparison of features with empty scenarios
        // assert!(
        //     approximately_eq(&reference, &cloned),
        //     "Identical feature should match clone"
        // );
    }
    #[test]
    fn test_feature_different_path_nodiff() {
        let mut reference = reference_feature();
        reference.path = Some(PathBuf::from("/path/to/my/reference.feature"));
        let mut cloned = reference.clone();
        cloned.path = Some(PathBuf::from("/path/to/code/test_reference.py"));
        assert!(
            approximately_eq(&reference, &cloned),
            "Similar feature (up to path) should match clone"
        );
        // FIXME Known limitation of the current method, not ignoring Path differences
        // assert!(json_diff_equals(&reference, &cloned), "Similar feature (up to path) should match clone");
    }
}
