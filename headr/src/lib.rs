use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>,
}

pub fn get_args() -> MyResult<Config> {

    Ok(Config {
        files: vec![],
        lines: 0,
        bytes: None
    })
}

pub fn run() -> MyResult<()> {

    Ok(())
}