use std::{fs::read_dir, path::Path, io::Result};
use mpris::Metadata;
use urlencoding::decode;

/// based on currently playing file, find corresponding .lrc-file if it exists near the song file
fn get_lyric_file(metadata: Metadata) -> Option<String> {
    let url: String = metadata.url()?.into();
    if url.starts_with("file://") {
        let url: String = match decode(&url) {
            Ok(url) => url.into_owned(),
            _ => { return None; }
        };
        let path = Path::new(url.trim_start_matches("file://"));
        let file_name: String = format!("{}.lrc", path.file_stem()?.to_str()?);
        assert!(path.is_absolute());
        match search_dir(file_name, path.parent()?, 1) {
            Ok(path) => return Path::new(&path),
            Err(_) => {
                match search_dir(file_name, path.parent()?.parent()?, 1) {
                    Ok(path) => return Path::new(&path),
                    Err(_) => return None
                }
            }
        }; // Search for the song file, this could probably look a lot better
    }
    return None
}

/// search the directory for given file_name with given recursive depth
fn search_dir(file_name: String, dir: &Path, depth: u16) -> Result<String, String> {
    if dir.is_dir() {
        for entry in read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() && depth > 0 {
                match search_dir(file_name, &path, depth-1) {
                    Ok(found) => Ok(found),
                    Err(_) => continue
                };
            } else {
                let file: String = path.file_name()
                    .unwrap()
                    .to_os_string()
                    .into_string()
                    .unwrap();
                if file == file_name {
                    let found: String = path.to_str().unwrap_or_default().into();
                    Ok(found)

                }
            }
        }
    }
    Err("".into())
}

pub fn get_current_lyrics() {

}
