use std::{collections::VecDeque, path::PathBuf, sync::LazyLock, fs};
use regex::Regex;
use crate::Lyric;

// TODO: these functions can and should probably be combined somehow
static RE_ELRC_TXT: LazyLock<Regex> = LazyLock::new(
    || Regex::new(r"(<([^>]+)>(.*?)(?=<|$))").unwrap());
static RE_LRC_TXT: LazyLock<Regex> = LazyLock::new(
    || Regex::new(r"(\[(\d{1,2}):(\d{1,2})\.(\d{2,3})(?:\.(\d{2,3}))?\](.*))").unwrap());
static RE_LRC_TIME: LazyLock<Regex> = LazyLock::new(
    || Regex::new(r"((\d{1,2}):(\d{1,2})\.(\d{2,3}))").unwrap());

// perhaps useless function, either do the whole for loop in the parse_lyric_file or use this for
// parsing a single line
fn re_get_text(haystack: &str) -> Lyric {
    let is_elrc: bool = RE_ELRC_TXT.is_match(haystack);
    if is_elrc {}
    todo!();
}



pub fn parse_lyric_file(file: PathBuf) -> Option<VecDeque<Lyric>> {
    let lyrics: VecDeque<Lyric>;
    let lines = fs::read_to_string(file)
        .expect("Should have been able to read the file");
        
    for line in lines.split("\n") {
        todo!();
        // for each line: find matches and read (time:text) from capture groups
        // handle enhanced lyrics and unicode DOM
        // append the Lyric to the Dequeue
    }

    return None
}
