use std::{fs::read_dir, path::Path, io::Result};
use mpris::Metadata;
use urlencoding::decode;


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
        search_dir(file_name, path.parent().unwrap());
    }
    return Some("".to_owned());
}

//
//    def _find_lrc(self, path: str, filename: str) -> str:
//        dirs, files = self._listdir_abs(path)
//        pdirs, pfiles = self._listdir_abs(path + "..")
//        for file in files + pfiles:
//            if os.path.basename(str(file)) == self._to_lrc(filename):
//                return str(file)
//        for dir_ in dirs + pdirs:
//            _, dirfiles = self._listdir_abs(dir_)
//            for file in dirfiles:
//                if os.path.basename(str(file)) == self._to_lrc(filename):
//                    return str(file)
//        return ""
//

/// non-recursively search a directory for a file with given file_name
fn search_dir(file_name: String, dir: &Path, depth: u16) -> Result<String, String> {
    if dir.is_dir() {
        for entry in read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() && depth > 0 {
                search_dir(file_name, &path, depth-1)?;
            } else {
                let file: String = path.file_name()
                    .unwrap()
                    .to_os_string()
                    .into_string()
                    .unwrap();
                if file == file_name {
                    let found: String = path.to_str().unwrap_or_default().into();
                    Ok(found);

                }
            }
        }
    }
    Err("".into());
}

    //let entries = read_dir(path)
    //    .map(|x| x.unwrap().path())
    //    .find(|x| x.file_stem().unwrap_or_default() == file_name)
    //    .to_str().unwrap().to_owned()


pub fn get_current_lyrics() {

}
