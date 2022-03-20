use std::fs;
use std::path::Path;

use gherkin;
use regex::Regex;

/// Computes equality of gherkin::Feature objects
/// in terms of Gherkin content
/// but ignoring the Spans (line/col number) where text was found
fn approximately_eq(reference: &gherkin::Feature, other: &gherkin::Feature) -> bool {
    let is_feature_equal: bool = reference.keyword == other.keyword
        && reference.name == other.name
        && reference.description == other.description
        && reference.background == other.background
        && reference.description == other.description;
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

/// Extract lines of text containing Gherkin keywords
///
/// Returned as a String with one Gherkin statement per line
fn extract_gherkin_text(text: &str) -> String {
    let gherkin_keywords = "Given|When|Then|And|But|Scenario|Background|Feature|In order to|As a|I want to|I need to|So that";
    let gherkin_regex_str = format!(r".*({})(.*)", gherkin_keywords);
    let gherkin_comments_regex = Regex::new(gherkin_regex_str.as_str()).unwrap();
    gherkin_comments_regex
        .captures_iter(text)
        .map(|cap| {
            format!(
                "{}{}\n",
                &cap[1], // newline-separated keyword+text concat
                // Remove Python docstring delimiters at the end of line
                &cap[2].trim_end_matches("\"\"\"")
            )
        })
        .collect()
}

fn main() {
    let feature_file = Path::new("features/checking_guess_valid_word.feature");
    let env = gherkin::GherkinEnv::new("en")
        .ok()
        .expect("Failed to get Gherkin environment");
    let feature_parsed = gherkin::Feature::parse_path(feature_file, env).unwrap();
    println!("Feature says:\n{:?}", feature_parsed);

    let test_file = Path::new("tests/test_checking_guess_valid_word.py");
    let test_text = fs::read_to_string(test_file).expect("Failed to read test file");
    let gherkin_test_matches = extract_gherkin_text(&test_text);
    println!("Test matches:\n{}", gherkin_test_matches);

    let test_env = gherkin::GherkinEnv::new("en")
        .ok()
        .expect("Failed to get Gherkin environment");
    // Assumes a single Feature per test file, assumes feature is present, no raw Scenarios
    let test_parsed = gherkin::Feature::parse(gherkin_test_matches, test_env).unwrap();
    println!("Test says:\n{:?}", test_parsed);
    println!(
        "Naive equality: {}\nCustom, approximate, match: {}",
        test_parsed == feature_parsed,
        approximately_eq(&test_parsed, &feature_parsed)
    );
}
