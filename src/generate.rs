use std::fs::File;
use std::path::PathBuf;
use tera::{Context, Tera};

pub enum SupportedTestTemplate {
    Rust,
    Python,
}

impl SupportedTestTemplate {
    fn get_template_filename(self) -> String {
        match self {
            SupportedTestTemplate::Rust => String::from("rust_test.rs"),
            SupportedTestTemplate::Python => String::from("py_test.py"),
        }
    }
}

pub fn generate(
    reference_feature: &gherkin::Feature,
    template: SupportedTestTemplate,
    path: PathBuf,
) {
    println!("Parsing all templates");
    let tera = match Tera::new("src/templates/*") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };

    let mut context = Context::new();
    context.insert("feature", &reference_feature);
    // context.insert("test_features", &project.parsed);
    println!("Generating test file...");
    let render_file = File::create(path).expect("Error creating file for writing");
    tera.render_to(&template.get_template_filename(), &context, render_file)
        .expect("Error expanding template");
}
