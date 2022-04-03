use grepkin::{approximately_eq, grep_parse_features_glob, parse_features_glob};

fn main() {
    let features = parse_features_glob("features/*.feature");
    let feature_ref = features.get(0).unwrap();
    println!("Feature file says:\n{:?}", &feature_ref);

    let code_features = grep_parse_features_glob("tests/*.py");
    let extracted_gherkin = code_features.get(0).unwrap();
    println!("Extracted:\n{:?}", &extracted_gherkin);

    // TODO assert Gherkin::parse_text(filter_gherkin_text(testfile)) == Gherkin::parse(testfile)
    println!(
        "Naive equality: {}\nCustom, approximate, match: {}",
        extracted_gherkin == feature_ref,
        approximately_eq(extracted_gherkin, feature_ref)
    );
}
