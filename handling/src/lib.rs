use std::path::Path;

pub fn open_or_create<P: AsRef<Path>>(path: &P, content: &str) {
    if !path.as_ref().exists() {
        std::fs::write(path, content).unwrap();
    }
}