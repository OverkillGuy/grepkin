use regex::Regex;
use std::ops::Range;

/// Gherkin DSL's english keywords
const GHERKIN_KEYWORDS: &str = "Given|When|Then|And|But|Scenario|Background|Feature|In order \
                                to|As a|I want to|I need to|So that";

/// Computes equality of gherkin::Feature objects
/// in terms of Gherkin content
/// but ignoring the Spans (line/col number) and Path where text was found
#[allow(clippy::nonminimal_bool)]
pub fn approximately_eq(reference: &gherkin::Feature, other: &gherkin::Feature) -> bool {
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
pub fn extract_gherkin_text(text: &str) -> String {
    let gherkin_regex_str = format!(r".*({})(.*)", GHERKIN_KEYWORDS);
    let gherkin_comments_regex = Regex::new(gherkin_regex_str.as_str()).unwrap();
    gherkin_comments_regex
        .captures_iter(text)
        .map(|cap| {
            format!(
                "{}{}\n",
                &cap[1], // newline-separated keyword+text concat
                // Remove Python docstring delimiters at the end of line
                &cap[2] // .trim_end_matches("\"\"\"")
            )
        })
        .collect()
}

/// Location of Gherkin keyword-description pairs
type GherkinRange = (Range<usize>, Range<usize>);
/// Locations of all Gherkin keyword-description pairs in a text
type GherkinRanges = Vec<GherkinRange>;

fn find_gherkin_text(text: &str) -> GherkinRanges {
    // Avoid Python docstring delimiters at the end of line = """
    let gherkin_regex_str = format!(r#".*({})(.*)(""")?"#, GHERKIN_KEYWORDS);
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

pub fn parse_features_glob(feature_path_glob: &str) -> Vec<gherkin::Feature> {
    let mut features_parsed: Vec<gherkin::Feature> = vec![];
    for feature_file_maybe in
        globwalk::glob(feature_path_glob).expect("Error globbing given features folder")
    {
        let feature_file = feature_file_maybe.expect("Error walking to given file in glob");
        let feature_parsed =
            gherkin::Feature::parse_path(feature_file.path(), gherkin::GherkinEnv::default())
                .expect("Error parsing file into Gherkin Feature");
        features_parsed.push(feature_parsed);
    }
    features_parsed
}
