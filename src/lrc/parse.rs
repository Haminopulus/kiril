use std::{sync::LazyLock, path::PathBuf};
use std::Regex::regex;
use crate::Lyric;

// TODO: these functions can and should probably be combined somehow
fn re_get_enhanced_text(haystack: &str) -> bool {
    static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(<([^>]+)>(.*?)(?=<|$))").unwrap());
    RE.is_match(haystack)
}

fn re_get_text(haystack: &str) -> bool {
    static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(\[(\d{1,2}):(\d{1,2})\.(\d{2,3})(?:\.(\d{2,3}))?\](.*))").unwrap());
    RE.is_match(haystack)
}
 
fn re_get_time(haystack: &str) -> bool {
    static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"((\d{1,2}):(\d{1,2})\.(\d{2,3}))").unwrap());
    RE.is_match(haystack)
}

pub fn parse_lyric_file(file: PathBuf) -> Option<VecDeque<Lyric>> {
    let lyrics: VecDeque<Lyric>;
    let contents = fs::read_to_string(file)
        .expect("Should have been able to read the file");

    // first: check if elrc, else normal lrc (how would you check for elrc? just check if the word
    // time stamps exist?)
    // for each line: find matches and read (time:text) from capture groups
    // append the Lyric to the Dequeue

    println!("With text:\n{contents}");
    return None
}
