use mpris::Metadata;
use urlencoding::decode;
use regex::Regex;


fn get_lyric_file(metadata: Metadata) {
    let path: &str = metadata.url().unwrap();
    // TODO: these do not need to be compiled every time, store them somewhere
    // FUNFACT: pee is stored in the balls
    let file_path = Regex::new(r"/[^/]+/.*").unwrap();
    let file_name = Regex::new(r"([^/]*$)").unwrap();
    // for now only local files are supported
    let result = file_path.find(path);
    if path.starts_with("file://") {
        let file_name: &str = path;  
    }
}

pub fn get_current_lyrics() {

}
