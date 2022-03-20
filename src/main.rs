use std::path::Path;
use std::fs;

use gherkin;
use regex::Regex;


fn main() {
    let feature_file = Path::new("features/checking_guess_valid_word.feature");
    let env = gherkin::GherkinEnv::new("en").ok().expect("Failed to get Gherkin environment");
    let parsed = gherkin::Feature::parse_path(feature_file, env);
    println!("Feature says:\n{:?}", parsed);

    let gherkin_keywords = "Given|When|Then|And|But|Scenario|Background|Feature|In order to|As a|I want to|I need to|So that";
    let gherkin_regex_str = format!(r".*({})(.*)", gherkin_keywords);
    let gherkin_comments_regex = Regex::new(gherkin_regex_str.as_str()).unwrap();

    let test_file = Path::new("tests/test_checking_guess_valid_word.py");
    let test_text = fs::read_to_string(test_file).expect("Failed to read test file");
    let gherkin_test_matches: String =
        gherkin_comments_regex.captures_iter(&test_text)
                              .map(|cap| format!("{}{}\n", &cap[1],
                                                 &cap[2].trim_end_matches(r#"""""#))) // Python docstring delimiters
                              .collect();
    // println!("Test matches:\n {}", gherkin_test_matches);

    let test_env = gherkin::GherkinEnv::new("en").ok().expect("Failed to get Gherkin environment");
    let test_parsed = gherkin::Feature::parse(gherkin_test_matches, test_env);
    println!("Test says:\n{:?}", test_parsed);
    // TODO: Match the gherkin::Feature's Vec<Scenarios> to the regex captures, giving percent match
}
