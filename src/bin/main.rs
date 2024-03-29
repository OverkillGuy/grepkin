use grepkin::compare::approximately_eq;
use grepkin::GherkinProject;

fn main() {
    // Parse the project's feature files + test code
    let project = GherkinProject::new(
        "features/checking_guess_valid*.feature".to_string(),
        "tests/*.py".to_string(),
    );
    println!("Project:\n{:?}", project);
    // Get the first of each (because in practice there is one of each, matching)
    let reference = project.references.iter().next().unwrap();
    let parsed = project.parsed.iter().next().unwrap();
    println!(
        "Ref glob: {}\nParsed glob: {}",
        project.reference_glob, project.parsed_glob
    );
    // Compare the reference to the actual features
    println!("Reference: {:?}\nParsed:    {:?}", &reference, &parsed);
    let feature_equals: bool = approximately_eq(parsed, reference); // Igores these mismatching, focus on content
    println!(
        "Naive equality: {}\nCustom, approximate, match: {}",
        parsed == reference, // Mismatching span + linecol + path
        feature_equals
    );
    std::process::exit(if feature_equals { 0 } else { 1 });
}
