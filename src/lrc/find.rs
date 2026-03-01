use std::{fs::read_dir, path::{Path, PathBuf}};
use mpris::Metadata;
use urlencoding::decode;

/// based on current metadata, find corresponding lrc-file if it exists near the song file
pub fn get_lrc_file(metadata: &Metadata) -> Option<PathBuf> {
    let url: String = metadata.url()?.into();

    if url.starts_with("file://") { // only handling local files
        let url: String = match decode(&url) {
            Ok(url) => url.into_owned(),
            _ => { return None; }
        };
        let path = Path::new(url.trim_start_matches("file://"));
        let file_name: &str = &format!("{}.lrc", path.file_stem()?.to_str()?);
        assert!(path.is_absolute());

        match search_dir(file_name, path.parent()?, 1) {
            Some(path) => return Some(PathBuf::from(&path)),
            None => return None
        };
    }
    return None
}

/// search the directory for given file_name with given recursive depth
fn search_dir(file_name: &str, dir: &Path, depth: u16) -> Option<String> {
    if dir.is_dir() {
        for entry in read_dir(dir).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                if depth > 0 {
                    match search_dir(file_name, &path, depth-1) {
                        Some(found) => return Some(found),
                        None => continue
                    };
                }
            } else if path.is_file() {
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


