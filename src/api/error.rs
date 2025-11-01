use std::io;

use thiserror::Error;

/// https://github.com/dtolnay/thiserror
/// https://crates.io/crates/thiserror
/// https://juejin.cn/post/7272005801081126968
/// https://www.shakacode.com/blog/thiserror-anyhow-or-how-i-handle-errors-in-rust-apps/
/// https://rustcc.cn/article?id=1e20f814-c7d5-4aca-bb67-45dcfb65d9f9
#[derive(Debug, Error)]
pub enum MyError {
    // file not exist error
    #[error("Error - {file} does not exist")]
    FileNotExistError{file: String},

    // create_dir_all error
    #[error("Error - fs::create_dir_all {dir_name}: {error}")]
    CreateDirAllError{dir_name: String, error: io::Error},

    // read file to string error
    #[error("Error - read {file}: {error}")]
    ReadFileError{file: String, error: io::Error},

    // write file error
    #[error("Error - fs::write {file}: {error}")]
    WriteFileError{file: String, error: io::Error},

    // para error
    #[error("Error - {para}")]
    ParaError{para: String},

    // io::Error
    #[error("I/O error occurred")]
    IoError(#[from] io::Error),
}
