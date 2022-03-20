use gherkin;
use std::path::Path;

fn main() {
    let file = Path::new("features/checking_guess_valid_word.feature");
    let env = gherkin::GherkinEnv::new("en").ok().expect("Failed to get Gherkin environment");
    let parsed = gherkin::Feature::parse_path(file, env);
    println!("Hello, world!\n{:?}", parsed);
}
