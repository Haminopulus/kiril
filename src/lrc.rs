use async_std::path::Path;
use mpris::Metadata;
use urlencoding::decode;
use regex::Regex;
use std::fs::read_dir;

//const PATH: Regex = Regex::new(r"/[^/]+/.*").unwrap();
//const FILE_NAME: Regex = Regex::new(r"([^/]*)\..*$").unwrap();

fn get_lyric_file(metadata: Metadata) {
    let url: &str = decode(metadata
        .url()
        .unwrap_or_default());

    if url.starts_with("file://") {
        let path = Path::new(url.trim_start_matches("file://"));
        let file_name: &str = path
            .file_stem()
            .unwrap_or_default();
        if !file_name.is_empty() {
            // find lrc file in parent-, current or subdirectories
            todo!();
            //read_dir(path)
            //    .unwrap().find(|x| x.unwrap().path().file_name() == file_name+".lrc");
        }
    }
}


pub fn get_current_lyrics() {

}
