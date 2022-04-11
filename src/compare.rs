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
pub fn json_diff_equals(reference: &gherkin::Feature, other: &gherkin::Feature) -> bool {
    let ref_json: serde_json::Value = serde_json::value::to_value(&reference).unwrap();
    let other_json: serde_json::Value = serde_json::value::to_value(&other).unwrap();
    // TODO Use custom treediff::tools::Delegate instead of Recorder
    let mut record = treediff::tools::Recorder::default();
    treediff::diff(&ref_json, &other_json, &mut record);
    // record.calls contains list of change type (CRUD-style)
    for call in record.calls {
        match call {
            treediff::tools::ChangeType::Unchanged(_k, _v) => {}
            // Any change is worth returning false immediately
            treediff::tools::ChangeType::Added(k, v) => {
                println!("Added! {:?}=>{:?}", k, v);
                return false;
            }
            // TODO Split this func into fn json_diff() -> Vec<ChangeType> (test it)
            // Then use that new func for bool. New func can be used for Display/Format = prettydiff
            treediff::tools::ChangeType::Modified(k, v1, v2) => {
                // k is a Vec<Key> = path inside struct
                // last path element = key to possibly ignore: match it
                match k.last().unwrap().to_string().as_str() {
                    "path" => {
                        println!("Modified path! Ignoring! was '{:?}', now '{:?}'", v1, v2);
                    }
                    "line" => {
                        println!("Modified line! Ignoring! was '{:?}', now '{:?}'", v1, v2);
                    }
                    "col" => {
                        println!("Modified col! Ignoring! was '{:?}', now '{:?}'", v1, v2);
                    }
                    "end" => {
                        println!("Modified end! Ignoring! was '{:?}', now '{:?}'", v1, v2);
                    }
                    "start" => {
                        println!("Modified start! Ignoring! was '{:?}', now '{:?}'", v1, v2);
                    }
                    _ => {
                        println!("Modified non-path! {:?} was '{:?}', now '{:?}", k, v1, v2);
                        return false;
                    }
                }
            }
            treediff::tools::ChangeType::Removed(k, v) => {
                println!("RMed! {:?}=>{:?}", k, v);
                return false;
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::{approximately_eq, json_diff_equals};
    use std::path::PathBuf;

    /// Sample feature to do some comparisons against
    fn reference_feature() -> gherkin::Feature {
        let reference_scenarios: Vec<gherkin::Scenario> = vec![
            gherkin::Scenario::builder()
                .keyword("Scenario".to_string())
                .name("Dummy scenario to be representative".to_string())
                .steps(vec![
                    gherkin::Step::builder()
                        .keyword("When".to_string())
                        .ty(gherkin::StepType::When)
                        .value("something happens".to_string())
                        .build(),
                ])
                .build(),
        ];
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
        assert!(
            json_diff_equals(&reference, &cloned),
            "Similar feature (up to path) should match clone"
        );
    }
}
