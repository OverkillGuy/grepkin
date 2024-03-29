use regex::Regex;
use std::collections::HashSet;
use std::ops::Range;
use std::{fs, path};

/// Gherkin DSL's english keywords
/// A little extended with Connextra templates
const GHERKIN_KEYWORDS: &str = "Given|When|Then|And|But|Scenario|Background|Feature|In order \
                                to|As a|I want to|I need to|So that";

// TODO assert Gherkin::parse_text(filter_gherkin_text(testfile)) == Gherkin::parse(testfile)

/// Extract lines of text containing Gherkin keywords
///
/// Returned as a String with one Gherkin statement per line
#[allow(dead_code)]
pub fn extract_gherkin_text(text: &str) -> String {
    let gherkin_regex_str = format!(r".*({})(.*)", GHERKIN_KEYWORDS);
    let gherkin_comments_regex = Regex::new(gherkin_regex_str.as_str()).unwrap();
    gherkin_comments_regex
        .captures_iter(text)
        .map(|cap| {
            format!(
                "{}{}\n",
                &cap[1], // newline-separated keyword+text concat
                &cap[2]
            )
        })
        .collect()
}

/// Location of Gherkin keyword-description pairs
type GherkinRange = (Range<usize>, Range<usize>);
/// Locations of all Gherkin keyword-description pairs in a text
type GherkinRanges = Vec<GherkinRange>;

/// Compute which slices of the text contain Gherkin words
///
/// Using regex, get keyword + description as separate ranges
fn find_gherkin_text(text: &str) -> GherkinRanges {
    let gherkin_regex_str = format!(r#".*({})(.*)"#, GHERKIN_KEYWORDS);
    let gherkin_comments_regex = Regex::new(gherkin_regex_str.as_str()).unwrap();
    gherkin_comments_regex
        .captures_iter(text)
        .map(|cap| {
            (
                Range {
                    start: cap.get(1).unwrap().start(),
                    end: cap.get(1).unwrap().end(),
                },
                Range {
                    start: cap.get(2).unwrap().start(),
                    end: cap.get(2).unwrap().end(),
                },
            )
        })
        .collect()
}

/// Show ranges of text where gherkin is NOT present, given Gherkin ranges
/// Used to know where to blank out into whitespace
/// for filter_gherkin_text
fn opposite_range_text(gherkin_locs: &GherkinRanges, text_size: usize) -> Vec<Range<usize>> {
    let mut index: usize = 0;
    let mut acc: Vec<Range<usize>> = vec![];
    // Strings look like:
    // `0???[kw.start..kw.end]??[desc.start..desc.end]??END`
    // And we want to grab `[0..kw.start] + [kw.end..desc.start] + [desc.end..END]`
    for (kw, desc) in gherkin_locs {
        // 0-sized rangeguard to avoid contiguous gherkin ranges
        if index != kw.start {
            acc.push(Range {
                start: index,
                end: kw.start,
            });
        }
        if kw.end != desc.start {
            acc.push(Range {
                start: kw.end,
                end: desc.start,
            });
        }
        index = desc.end;
    }
    if index != text_size {
        acc.push(Range {
            start: index,
            end: text_size,
        });
    }
    acc
}

/// Filter string ranges containing Gherkin keywords
///
/// Returned is a string with same length as original, with all non-gherkin
/// content replaced by whitespace with correct amount of newlines
///
/// This is important to make the Feature::{Span, Linecol} accurate.
pub fn filter_gherkin_text(text: &str) -> String {
    let gherkin_locs = find_gherkin_text(text);
    let nongherkin_locs = opposite_range_text(&gherkin_locs, text.len());
    // Now we have location of gherkin, we want to replace everything else with whitespace.
    // To conserve correct line-column numbers (file positioning), we need to find all newlines
    // To keep them though
    // Get all newline locations the given string
    let newline_locs: Vec<usize> = text.match_indices('\n').map(|m| m.0).collect();
    let mut filled_text: String = String::from(text);
    // Reset to spacechar all non-gherkin loc
    for nongherkin_loc in nongherkin_locs {
        let whitespace_repeat_size = nongherkin_loc.end - nongherkin_loc.start;
        filled_text.replace_range(nongherkin_loc, &" ".repeat(whitespace_repeat_size));
    }
    // Then re-position the newline characters
    for newline_loc in newline_locs {
        filled_text.replace_range(newline_loc..(newline_loc + 1), "\n");
    }
    // Finally remove the triple-doublequotes = python docstrings
    filled_text // .replace("\"\"\"", "   ")
}

/// Remove trailing Python docstrings-marker in Feature fields' descriptions
///
/// Docstrings are parsed badly due to regex-matching all non-whistepace on a gherkin keyword
pub fn fix_docstrings(feature: gherkin::Feature) -> gherkin::Feature {
    let mut cleaned = feature;
    for scenario in cleaned.scenarios.iter_mut() {
        let docstring_marker = "\"\"\"";
        if scenario.name.ends_with(docstring_marker) {
            let trimmed_name: &str = scenario.name.trim_end_matches(docstring_marker);
            scenario.name = trimmed_name.to_string();
        }
    }
    cleaned
}

/// Parse Gherkin features files given a glob
///
/// Parse using default Gherkin parser, suitable for actual Feature files, not code
pub fn parse_features_glob(feature_path_glob: &str) -> HashSet<gherkin::Feature> {
    let mut features_parsed: HashSet<gherkin::Feature> = HashSet::new();
    for feature_file_maybe in
        globwalk::glob(feature_path_glob).expect("Error globbing given features folder")
    {
        let feature_file = feature_file_maybe.expect("Error walking to given file in glob");
        let feature_parsed =
            gherkin::Feature::parse_path(feature_file.path(), gherkin::GherkinEnv::default())
                .expect("Error parsing file into Gherkin Feature");
        features_parsed.insert(feature_parsed);
    }
    features_parsed
}

/// Parse Gherkin bits of files given a glob
///
/// Greps for Gherkin keywords to support code files with comments-based Gherkin
pub fn grep_parse_features_glob(code_path_glob: &str) -> HashSet<gherkin::Feature> {
    let mut features_parsed: HashSet<gherkin::Feature> = HashSet::new();
    for code_file_maybe in
        globwalk::glob(code_path_glob).expect("Error globbing given features folder")
    {
        let code_file = code_file_maybe.expect("Error walking to given file in glob");
        let feature_parsed = parse_gherkin_grep_file(code_file.path());
        features_parsed.insert(feature_parsed);
    }
    features_parsed
}

/// Parse a text into a Gherkin Feature, via regex
///
/// Greps for Gherkin keywords, filtering out the non-gherkin lines into a dummy
/// string, to give an accurate Span/Linecol fields
pub fn parse_gherkin_grep(text: &str) -> gherkin::Feature {
    fix_docstrings(
        gherkin::Feature::parse(filter_gherkin_text(text), gherkin::GherkinEnv::default())
            .expect("Unable to parse gherkin from filtered keywords of code"),
    )
}

/// Parse a single Gherkin code file given its path
///
/// Filters it via regex, to support (code) files that would fail parsing via
/// the default gherkin module's parser, while keeping accurate Span/Linecol,
/// which extract_gherkin_text would bork (by not keeping non-gherkin file
/// structure)
pub fn parse_gherkin_grep_file(file_path: &path::Path) -> gherkin::Feature {
    let file_text = fs::read_to_string(file_path).expect("Failed to read text file");
    fix_path(parse_gherkin_grep(&file_text), file_path.to_path_buf())
}

/// Reset a Feature's path to a specific filepath
///
/// The path won't be set right when we parse "raw string" instead of file
/// as we do when using `filter_gherkin_text`.
fn fix_path(feature: gherkin::Feature, file_path: path::PathBuf) -> gherkin::Feature {
    let mut cloned = feature;
    cloned.path = Some(file_path);
    cloned
}
