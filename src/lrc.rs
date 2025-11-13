use std::{collections::VecDeque, fs::{self, read_dir}, path::{Path, PathBuf}};
use mpris::Metadata;
use regex::Regex;
use urlencoding::decode;
use std::sync::LazyLock;

use crate::Lyric;


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

/// based on currently playing file, find corresponding .lrc-file if it exists near the song file
fn get_lyric_file(metadata: Metadata) -> Option<PathBuf> {
    let url: String = metadata.url()?.into();
    if url.starts_with("file://") {
        let url: String = match decode(&url) {
            Ok(url) => url.into_owned(),
            _ => { return None; }
        };
        let path = Path::new(url.trim_start_matches("file://"));
        let file_name: &str = &format!("{}.lrc", path.file_stem()?.to_str()?);
        assert!(path.is_absolute());
        match search_dir(file_name, path.parent()?, 1) {
            Some(path) => return Some(PathBuf::from(&path)),
            None => {
                match search_dir(file_name, path.parent()?.parent()?, 1) {
                    Some(path) => Some(PathBuf::from(&path)),
                    None => return None
                }
            }
        }; // Search for the song file, this could probably look a lot better
    }
    return None
}

fn search_dir(file_name: &str, dir: &Path, depth: u16) -> Option<String> {
/// search the directory for given file_name with given recursive depth
    if dir.is_dir() {
        for entry in read_dir(dir).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() && depth > 0 {
                match search_dir(file_name, &path, depth-1) {
                    Some(found) => return Some(found),
                    None => continue
                };
            } else {
                let file: String = path.file_name()
                    .unwrap()
                    .to_os_string()
                    .into_string()
                    .unwrap();
                if file == file_name {
                    let found: String = path.to_str().unwrap_or_default().into();
                    return Some(found)
                }
            }
        }
    }
    return None
}

// this should probably be a separate file
fn parse_lyric_file(file: PathBuf) -> Option<VecDeque<Lyric>> {
    let lyrics: VecDeque<Lyric>;
    let contents = fs::read_to_string(file)
        .expect("Should have been able to read the file");


    println!("With text:\n{contents}");
    return None
}

pub fn get_current_lyrics(metadata: Metadata) -> VecDeque<Lyric> {
    let lyric_file : PathBuf;
    match get_lyric_file(metadata) {
        Some(pathbuf) => {
            println!("{}", pathbuf.to_str().unwrap());
            lyric_file = pathbuf;
        },
        _ => unimplemented!()
    };
    let lyrics: VecDeque<Lyric>;
    match parse_lyric_file(lyric_file) {
        Some(lrcs) => {
            println!("Got Lyrics of length {}", lrcs.len());
            lyrics = lrcs;
        }
        _ => unimplemented!()
    };
    return lyrics
}
