use grepkin::{approximately_eq, GherkinProject};

fn main() {
    // Parse the project's feature files + test code
    let project = GherkinProject::new("features/*.feature".to_string(), "tests/*.py".to_string());
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
    println!(
        "Naive equality: {}\nCustom, approximate, match: {}",
        parsed == reference,                 // Mismatching span + linecol + path
        approximately_eq(parsed, reference)  // Igores these mismatching, focus on content
    );
}
