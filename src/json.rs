use serde::Serialize;
use std::collections::VecDeque;
use crate::Lyric;

#[derive(Serialize)]
struct JsonLyrics {
    prev_lines: Vec<String>,
    next_lines: Vec<String>,
    prev_words: String,
    curr_word: String,
    next_words: String,
    cover: String
}

/// take extracted values and put them into a JSON string
fn to_json(prev_lines: Vec<String>, next_lines: Vec<String>, prev_words: String, curr_word: String, next_words: String, cover: String) -> String {
    let obj = JsonLyrics{
        prev_lines: prev_lines,
        next_lines: next_lines,
        prev_words: prev_words,
        curr_word: curr_word,
        next_words: next_words,
        cover: cover
    };
    serde_json::to_string(&obj).unwrap()
}

/// extracts strings from timestamps and returns JSON string
///  * lyrics: VecDeque<Lyric> = the lyrics in Timestamp format
///  * line_num: u32 = number of current line
///  * word_num: u32 = number of current word in the line
///  * step: u32 = how many lines before and after should be processed
pub fn json_convert(lyrics: &VecDeque<Lyric>, line_num: u32, word_num: u32, step: u32, cover: String) -> String {
    let mut json_prev_lines: Vec<String> = Vec::default();
    let mut json_next_lines: Vec<String> = Vec::default();
    let mut json_curr_word: String = String::default();
    let mut json_prev_words: String = String::default();
    let mut json_next_words: String = String::default();

    for i in line_num as i64 - step as i64 .. (line_num + step + 1) as i64 {
        // weird maybe
        if i < 0 {json_prev_lines.push("".into()); continue;} 
        else if i >= lyrics.len() as i64 {json_next_lines.push("".into()); continue;}
        let i = i as u32;

        let line = lyrics.get(i as usize).unwrap();
        // previous and next lines do not need to be split
        if i != line_num {
            let mut line_str = String::default();
            for timestamp in line {
                if !timestamp.1.trim().is_empty() {
                    line_str += &timestamp.1;
                    line_str += " ";
                }
            }
            if i < line_num {
                json_prev_lines.push(line_str.trim().to_owned());
            } else {
                json_next_lines.push(line_str.trim().to_owned());
            }
        // split the current line into prev, curr and next words
        } else if i == line_num {
            // elrc has multiple time stamps
            if line.len() > 1 {
                for j in 0..word_num as usize {
                    let word = &line.get(j).unwrap().1;
                    if !word.trim().is_empty() {
                        json_prev_words += word;
                        json_prev_words += " ";
                    }
                }
                // incase the word is just a whitespace: we also take the next word, which makes
                // more sense because you cannot sing a whitespace (probably)
                let mut overhang = 0;
                json_curr_word += match line.get(word_num as usize) {
                    Some(word) => if !word.1.trim().is_empty() {&word.1} else {
                        let next = match &line.get((word_num + 1) as usize) {
                            Some(tag) => &tag.1,
                            None => ""
                        };
                        if !next.trim().is_empty() {overhang = 1; &next} else {""}
                    },
                    None => ""
                };
                json_next_words += " ";
                for j in (word_num + 1 + overhang) as usize..line.len() {
                    let word = &line.get(j).unwrap().1;
                    if !word.trim().is_empty() {
                        json_next_words += word;
                        json_next_words += " ";
                    }
                }
            // no splitting for normal lrc files
            } else {
                json_curr_word += &line.get(0).unwrap().1;
            }
        }
    }
    to_json(json_prev_lines, json_next_lines, json_prev_words, json_curr_word, json_next_words, cover.into())
}
