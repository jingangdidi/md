use mkd::{
    parse_paras::parse_para,
    markdown::render_md,
    error::MyError,
};

fn main() {
    if let Err(e) = run() {
        println!("{}", e);
    }
}

fn run() -> Result<(), MyError> {
    // parse para
    let paras = parse_para()?;

    // render
    render_md(paras.file, &paras.language, paras.width, &paras.outpath)
}
