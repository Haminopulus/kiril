use std::{collections::VecDeque, path::PathBuf};
use mpris::Metadata;
use crate::{lrc::{find::get_lrc_file, parse::parse_lrc_file}, Lyric};

pub mod find;
pub mod parse;

pub fn get_lyrics(metadata: &Metadata) -> Option<VecDeque<Lyric>>{
    let lyric_file : PathBuf;
    // find corresponding sidecar file
    match get_lrc_file(metadata) {
        Some(pathbuf) => {
            println!("{}", pathbuf.to_str().unwrap());
            lyric_file = pathbuf;
        },
        _ => return None
    };

    // parse file to extract lyrics
    match parse_lrc_file(lyric_file) {
        Some(lrcs) => {
            println!("DEBUG: Got Lyrics of with {} lines", lrcs.len());
            return Some(lrcs)
        }
        _ => return None
    };
}
