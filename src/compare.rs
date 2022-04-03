/// Computes equality of gherkin::Feature objects in terms of Gherkin content
/// but ignoring the Spans (line/col number) and Path where text was found.
///
/// Workaround for Feature equality operator being too specific when comparing
/// the same feature, parsed from different sources (proper feature file vs
/// extracted from test code).
/// TODO Prove this function really works via tests
/// FIXME: zip iter "reference" exhaustion causes other-exclusive scenarios+steps to wrongly match

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
