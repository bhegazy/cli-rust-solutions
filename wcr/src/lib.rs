use core::num;
use std::{error::Error, io::{BufReader, BufRead, self, Read}, fs::File};
use clap::{Command, Arg};

type MyResult<T> =  Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: bool,
    words: bool,
    bytes: bool,
    chars: bool,
}
#[derive(Debug, PartialEq)]
pub struct FileInfo {
    num_lines: usize,
    num_words: usize,
    num_bytes: usize,
    num_chars: usize,
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("wcr")
    .version("0.0.1")
    .author("Bill Hegazy <bill.hegazy@gmail.com>")
    .about("Rust wc")
    .arg(
        Arg::new("files")
        .value_name("FILE")
        .help("Input file(s)")
        .multiple_occurrences(true)
        .allow_invalid_utf8(true)
        .default_value("-"),
    )
    .arg(
        Arg::new("lines")
        .value_name("LINES")
        .help("Show line count")
        .short('l')
        .long("lines")
        .takes_value(false)
    )
    .arg(
        Arg::new("words")
        .value_name("WORDS")
        .help("Show words count")
        .short('w')
        .long("words")
        .takes_value(false)
    )
    .arg(
        Arg::new("bytes")
        .value_name("BYTES")
        .help("Show byte count")
        .short('c')
        .long("bytes")
        .takes_value(false)
    )
    .arg(
        Arg::new("chars")
        .value_name("CHARS")
        .help("Show character count")
        .short('m')
        .long("chars")
        .takes_value(false)
        .conflicts_with("bytes")
    )
    .get_matches();
    let ( mut lines,
        mut words,
        mut bytes,
        chars,
    ) = (
        matches.is_present("lines"),
        matches.is_present("words"),
        matches.is_present("bytes"),
        matches.is_present("chars"),
    );
    if !lines && !words && !bytes && !chars {
        lines = true;
        words = true;
        bytes = true;
    }
    // if [lines, words, bytes, chars].iter().all(|v| !v) {
    //     (lines, words, bytes) = (true, true, true);
    // }

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        lines,
        words,
        bytes,
        chars,
    })
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn count(mut file: impl BufRead) -> MyResult<FileInfo> {
    let mut num_lines = 0;
    let mut num_words = 0;
    let mut num_bytes = 0;
    let mut num_chars = 0;
    
    // count bytes
    let mut buffer = Vec::new();
    let read = file.read(&mut buffer)?;
    println!("{}", String::from_utf8_lossy(&buffer[..read]));

    // count words
    // for (n, test) in file.split(b' '){
    //     num_words += n + 1;
    //     println!("{:?}", test)
    // }

    // count lines
    for (n, line) in file.lines().enumerate() {
        num_lines += n + 1;
        // let mut v: Vec<&str> = Vec::new();
        // v = line.unwrap().split(' ').collect();
        // num_words = v.len()
        for (x, word) in line.unwrap().split(' ').enumerate() {
            num_words = x+1;
            println!("{}: {}: {}", x, word, num_words);
        }
    };
    // num_bytes = file.bytes().count();
    Ok(FileInfo{
        num_lines,
        num_words,
        num_bytes,
        num_chars,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("{}: {}", err, filename),
            Ok(file) => {
                let line_num = count(file);
                println!("Opened {} line_num: {:#?}", filename, line_num);
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use crate::{count, FileInfo};
    
    #[test]
    fn test_count() {
        let text = "Be careful for what you wish for, cause you just might get it all. \r\n";
        let info = count(Cursor::new(text));
        assert!(info.is_ok());
        let expected = FileInfo {
            num_lines: 1,
            num_words: 15,
            num_bytes: 72,
            num_chars: 72,
        };
        assert_eq!(info.unwrap(), expected);
    }
}