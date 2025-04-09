use std::path::Path;
use std::fs::OpenOptions;
use std::io::Write;

pub fn open_or_create<P: AsRef<Path>>(path: &P, content: &str) {
    // if file exists, open it, otherwise create it
    if path.as_ref().exists() {
        return;
    }

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path)
        .expect("Failed to open or create file");

    file.write_all(content.as_bytes()).expect("Failed to write to file");    
}