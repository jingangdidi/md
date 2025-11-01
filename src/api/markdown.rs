use std::fs::{write, read_to_string};
use std::path::{Path, PathBuf};

use crate::error::MyError;

const MARKDOWN: &str = include_str!("../../assets/markdown.html");

/// create html file
pub fn render_md(files: Vec<PathBuf>, language: &str, width: u8, outpath: &Path) -> Result<(), MyError> {
    for f in files {
        match read_to_string(&f) {
            Ok(content) => {
                let mut html = MARKDOWN.replace("let content = ``;", &format!("let content = `{}`;", content.replace("`", "\\`")));
                if language != "en" {
                    html = html.replace("lang: 'en'", &format!("lang: '{}'", language));
                }
                if width != 60 {
                    html = html.replace("width: 60%", &format!("width: {}%", width));
                }
                let html_file_name = file_name(&f)+".html";
                let file_path = outpath.join(&html_file_name);
                if let Err(e) = write(&file_path, html) {
                    return Err(MyError::WriteFileError{file: file_path.to_str().unwrap().to_string(), error: e})
                }
            },
            Err(e) => return Err(MyError::ReadFileError{file: f.to_str().unwrap().to_string(), error: e}),
        }
    }
    Ok(())
}

/// get file name (non-extension)
fn file_name(file: &Path) -> String {
    match file.file_stem() {
        Some(n) => n.to_str().unwrap().to_string(),
        None => file.to_str().unwrap().to_string(),
    }
}
