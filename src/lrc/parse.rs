use std::{collections::VecDeque, fs, path::PathBuf, sync::LazyLock, time::Duration};
use regex::Regex;
use crate::Lyric;

static RE_ELRC_TXT: LazyLock<Regex> = LazyLock::new(
    || Regex::new(r"<(\d{1,2}:\d{1,2}\.\d{2,3})> ([^< \n]*)").unwrap());
static RE_LRC_TXT: LazyLock<Regex> = LazyLock::new(
    || Regex::new(r"\[\d{1,2}:\d{1,2}\.\d{2,3}\](.*)").unwrap());
static RE_LRC_TIME: LazyLock<Regex> = LazyLock::new(
    || Regex::new(r"^\[(\d{1,2}:\d{1,2}\.\d{2,3})\]").unwrap());
static RE_TIMESTAMP: LazyLock<Regex> = LazyLock::new(
    || Regex::new(r"(\d{1,2}):(\d{1,2})\.(\d{2,3})").unwrap());

// parsing a timestamp with format mm:ss.xx 
// where mm is minutes, ss is seconds and xx is centiseconds
//
fn parse_timestamp(timestamp: &str) -> Duration {
    let mut ttt: bool = false;
    let captures = RE_TIMESTAMP.captures(timestamp).unwrap();
    let (_, mmssxx): (&str, [&str;3]) = captures.extract();
    let (mm, ss, xx): (u64, u64, u32) = (
        mmssxx[0].parse().unwrap(),
        mmssxx[1].parse().unwrap(),
        {
            if mmssxx[2].len() == 3 {ttt = true;}
            mmssxx[2].parse().unwrap()
        }
    );
    Duration::new(mm * 60 + ss, xx * (if ttt {1_000_000} else {10_000_000}))
}


pub fn parse_lrc_file(file: PathBuf) -> Option<VecDeque<Lyric>> {
    let mut lyrics: VecDeque<Lyric> = VecDeque::new();
    let lines: String = fs::read_to_string(file).unwrap();

    let is_elrc: bool = RE_ELRC_TXT.is_match(&lines);
    for line in lines.split("\n") {

        if !RE_LRC_TIME.is_match(line) {continue;}
        let mut line_lyric: Lyric = VecDeque::new();
        let line_start = RE_LRC_TIME.captures(line).unwrap()
            .get(1).unwrap()
            .as_str();

        if is_elrc {
            line_lyric.push_back((parse_timestamp(&line_start),String::new()));
            for timetag in RE_ELRC_TXT.captures_iter(line) {
                let (_, [time, word]) = timetag.extract();
                let word = if word.is_empty() {" "} else {word};
                line_lyric.push_back(
                    (parse_timestamp(time), word.to_owned())
                );
            }
        } else {
            let line_txt: &str = RE_LRC_TXT.captures(line)
                .unwrap()
                .extract::<1>().1[0]; // access the 2md element of the tuple and the 1st element
                                      // of the capture group array
            line_lyric.push_back((parse_timestamp(line_start), line_txt.into()));
        }
        lyrics.push_back(line_lyric);
    }

    return Some(lyrics)
}
