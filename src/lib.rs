pub mod compare;
mod parse;

use std::collections::HashSet;

/// A project's worth of Gherkin features as parsed, split between actual (from
/// tests) and reference. Assumed parsed via globs
#[derive(Debug, Clone)]
pub struct GherkinProject {
    pub references: HashSet<gherkin::Feature>,
    pub parsed: HashSet<gherkin::Feature>,
    pub reference_glob: String,
    pub parsed_glob: String,
}

impl GherkinProject {
    pub fn new(reference_glob: String, parsed_glob: String) -> GherkinProject {
        let reference_gherkins: HashSet<gherkin::Feature> =
            parse::parse_features_glob(&reference_glob);
        let parse_gherkins: HashSet<gherkin::Feature> =
            parse::grep_parse_features_glob(&parsed_glob);
        GherkinProject {
            references: reference_gherkins,
            parsed: parse_gherkins,
            reference_glob,
            parsed_glob,
        }
    }
}

// pub enum GrepkinFeatureType {
//     Referenced,
//     Extracted,
// }

// pub struct GrepkinFeature{
//     feature: gherkin::Feature,
//     source: GrepkinFeatureType,
// }

// Match library idea:
//
// - Merge lists of features via GrepkinFeature (convert to Array of Structure)
// - Allocate HashMap<String, Vec<GrepkinFeature> based on feature.name string
// - [FUTURE] If features non-allocated, do string similarlity
// - Check each HashMap key has 2 values: 1 source.Referenced + 1 source.Extracted
// - If these pairs exist, inspect them pairwise
// - Pairwise check description
// - Pairwise descent into scenarios
// - Scenarios HashMap<String, Vec<GrepkinScenario>> and do same
// - Same down one more to Steps (group by step types for similarity)
// - Output list of missing:
//
//
// pub enum GrepkinChangeTag {
// // Compared to Referenced feature
//     Added,
//     Missing,
//     // [FUTURE] Similarity checking:
//     Modified,
//     Reordered,
// }

// pub enum GrepkinDiffContent {
//     Feature(gherkin::Feature),
//     Scenario(gherkin::Scenario),
//     Step(gherkin::Step),
// }

// pub struct GrepkinDiff {
//     tag: GrepkinChangeTag,
//     reference: GrepkinDiffContent,
//     extracted: GrepkinDiffContent,
//     fieldname_diffed: Optional<String>,
// }

// pub struct GrepkinProjectDiff{
//     matches: bool,
//     diff: Vec<GrepkinDiff>
// }
