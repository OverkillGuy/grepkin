pub mod compare;
pub mod generate;
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
