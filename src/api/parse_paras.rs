use std::fs::create_dir_all;
use std::path::PathBuf;

use argh::FromArgs;

use crate::error::MyError;

#[derive(FromArgs)]
#[argh(help_triggers("-h", "--help"))] // https://github.com/google/argh/pull/106
/// render markdown, save as html file
struct Paras {
    /// markdown file, multiple files separated by commas
    #[argh(option, short = 'f')]
    file: String,

    /// show what language when copy code, support: en, es, pt-BR, fr, de, ja, ko, ru, zh, zh-tw, default: en
    #[argh(option, short = 'l')]
    language: Option<String>,

    /// width, (0, 100], default: 60
    #[argh(option, short = 'w')]
    width: Option<u8>,

    /// output path, default: ./
    #[argh(option, short = 'o')]
    outpath: Option<String>,
}

/// parsed para
///#[derive(Debug, Default)]
pub struct ParsedParas {
    pub file:     Vec<PathBuf>,
    pub language: String,
    pub width:    u8,
    pub outpath:  PathBuf,
}

/// parse para
pub fn parse_para() -> Result<ParsedParas, MyError> {
    let para: Paras = argh::from_env();
    Ok(ParsedParas{
        file: {
            let mut files: Vec<PathBuf> = Vec::new();
            for f in para.file.split(",") {
                let tmp_file = PathBuf::from(f);
                if !(tmp_file.exists() && tmp_file.is_file()) {
                    return Err(MyError::FileNotExistError{file: f.to_string()})
                }
                files.push(tmp_file);
            }
            files
        },
        language: match para.language {
            Some(lang) => {
                if !["en", "es", "pt-BR", "fr", "de", "ja", "ko", "ru", "zh", "zh-tw"].iter().any(|l| l == &lang) {
                    return Err(MyError::ParaError{para: format!("-l only support en, es, pt-BR, fr, de, ja, ko, ru, zh, zh-tw, not {}", lang)})
                }
                lang
            },
            None => "en".to_string(),
        },
        width: match para.width {
            Some(w) => {
                if w == 0 {
                    return Err(MyError::ParaError{para: "-w must > 0".to_string()})
                } else if w > 100 {
                    return Err(MyError::ParaError{para: format!("-w must <= 100, not {}", w)})
                }
                w
            },
            None => 60,
        },
        outpath: match para.outpath {
            Some(o) => {
                let tmp_path = PathBuf::from(&o);
                if !(tmp_path.exists() && tmp_path.is_dir()) {
                    if let Err(err) = create_dir_all(&tmp_path) {
                        return Err(MyError::CreateDirAllError{dir_name: o, error: err})
                    }
                }
                tmp_path
            },
            None => PathBuf::from("./"),
        },
    })
}
