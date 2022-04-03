use grepkin::{
    approximately_eq, extract_gherkin_text, filter_gherkin_text, fix_docstrings,
    parse_features_glob,
};

use std::{fs, path};

fn main() {
    let features = parse_features_glob("features/*.feature");
    let feature_ref = features.get(0);
    println!("Feature file says:\n{:?}", &feature_ref);
    // for feature_file_maybe in globwalk::glob("features/*.feature").unwrap() {
    //     let feature_file = feature_file_maybe.expect("Error getting the file");
    //     println!("Feature: '{}'", feature_file.file_name().to_str().unwrap());
    //     if feature_file.metadata().unwrap().is_dir() {
    //         println!("Iz dir: skip");
    //         continue;
    //     }
    //     let feature_parsed =
    //         gherkin::Feature::parse_path(feature_file.path(), gherkin::GherkinEnv::default())
    //             .unwrap();
    //     println!(
    //         "Feature file {} says:\n{:?}",
    //         feature_file
    //             .file_name()
    //             .to_str()
    //             .expect("Error getting feature file name"),
    //         feature_parsed
    //     );
    // }
    let test_file = path::Path::new("tests/test_checking_guess_valid_word.py");

    let test_text = fs::read_to_string(test_file).expect("Failed to read test file");

    let gherkin_filtered = filter_gherkin_text(&test_text);
    let extracted_gherkin_str = extract_gherkin_text(&gherkin_filtered);
    let extracted_gherkin = fix_docstrings(
        gherkin::Feature::parse(extracted_gherkin_str, gherkin::GherkinEnv::default()).unwrap(),
    );
    println!("Extracted: {:?}", &extracted_gherkin);

    let filtered_filepath = path::Path::new("tests/test_x.py");
    fs::write(filtered_filepath, &gherkin_filtered).expect("Error writing back filtered gherkin");

    let filtered_gherkin = fix_docstrings(
        gherkin::Feature::parse_path(filtered_filepath, gherkin::GherkinEnv::default())
            .expect("Error parsing filtered file into Gherkin Feature"),
    );

    println!("Filtered:  {:?}", &filtered_gherkin);
    // TODO assert Gherkin::parse_text(filter_gherkin_text(testfile)) == Gherkin::parse(testfile)
    println!(
        "Naive equality: {}\nCustom, approximate, match: {}",
        extracted_gherkin == filtered_gherkin,
        approximately_eq(&extracted_gherkin, &filtered_gherkin)
    );
}
