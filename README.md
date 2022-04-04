# Grepkin — Grep for Gherkin!

Extract Gherkin text from surrounding text, compare parsed Gherkin to reference
Feature files.

Useful as low-tech replacement for Cucumber: Write Gherkin in your test's
comments, and check that the reference Gherkin Features (in separate folder)
match all your tests!

Available as standalone binary, or pre-commit hook to enforce matching features
in your project.


Future versions will allow extracting test code's Gherkin features into
reference, or generating test code from given Feature references.


## Dependencies

-   [Rust](https://www.rust-lang.org/) `2021` edition
-   (Optional) [pre-commit](https://pre-commit.com) for setting up `grepkin` as a git hook
-   (Optional) [Docker](https://www.docker.com/) for running grepkin without installing Rust on system

## Installation & Usage

### As pre-commit hook

Pick your own repo, and try out the pre-commit hook from rust executable (requires rust toolchain):

    pre-commit try-repo /path/to/grepkin-repo grepkin --rev SOMECOMMITHASH --verbose --all-files

This makes a temporary Rust environment for the pre-commit hook, builds the
tool, and runs it just once (won't affect existing hooks, this is a one-off)


The same process works with a docker image, built live from the repo's Dockerfile (no Rust toolchain required, compiles in docker!):

    pre-commit try-repo /path/to/grepkin-repo grepkin --rev SOMECOMMITHASH  grepkin-dockerfile --verbose --all-files

If you want to see the pre-commit hook running on the current repo instead of
another's, take a look at the following `Makefile` targets:

    make run-precommit-rust
    make run-precommit-docker

### As a debian package

Build the debian package

    cargo deb

Install it on your machine:

    sudo dpkg -i target/debian/grepkin_0.1.0_amd64.deb

Use the tool:

    grepkin

Sample output (truncated the very long lines)

```
Project:
GherkinProject { references: {Feature { keyword: "Feature", name: "Checking a guess is a valid word"
Ref glob: features/*.feature
Parsed glob: tests/*.py
Reference: Feature { keyword: "Feature", name: "Checking a guess is a valid word", description: Some
Parsed:    Feature { keyword: "Feature", name: "Checking a guess is a valid word", description: Some
Naive equality: false
Custom, approximate, match: true
```

<!-- ### As a Docker container -->

<!-- The project's `Dockerfile` is mainly used for pre-commit hook, but can be used standalone. -->

<!-- Build it as a docker image: -->

<!--     make build-docker -->

<!-- and run as a docker container: -->

<!--     make run-docker -->


### As a Rust library

Parse everything under glob `features/*.feature*` as reference, and extract out
Gherkin from `tests/*.py`.

```rust
use grepkin::GherkinProject;

fn main() {
    // Parse the project's feature files + test code
    let project = GherkinProject::new("features/*.feature".to_string(), "tests/*.py".to_string());
    println!("Project:\n{:?}", project);
}
```

## Development

This project is written using Rust 2021 edition, using `cargo` as package
manager.

    cargo build

    cargo run

This project uses [pre-commit](https://pre-commit.com/) to enforce code guidelines, and has a
sample `Makefile` for ease of use; Run all those checks in one go with:

    make



## List of things TO DO

This project is far from being ready for broad use. Here's a list of missing
features I want to implement

-   Support for generating test-withcomments-from-feature (jinja templates)
-   Generate featurefile from test comments (SEE CUSTOM SERIALIZER FOR STRING)
-   Support scenario-only (feature-less) test files
-   Support set-enabled comparison of features/tests
-   Support out of order scenarios and steps (currently Vec = ordered)
-   CLI parsing
-   Ignore list for glob, from config file?
-   Testing (duh)
-   Sphinx-needs/mdbook style generation of features + link to test code
-   Use [pytest hooks](https://pytest.org/en/7.1.x/reference/reference.html#test-running-runtest-hooks) to collect test names (filepath, linenum, testname) to compare against gherkin

# License

This project uses GPL-v3-or-later license, see file `LICENSE.txt`
