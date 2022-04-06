use std::io::Read;
use std::{error::Error, io::{BufRead, BufReader, self}, fs::File};
use clap::{Command, Arg};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>,
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("headr")
    .version("0.0.1")
    .author("Bill Hegazy <bill.hegazy@gmail.com>")
    .about("Rust head Command")
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
        .help("Number of lines")
        .short('n')
        .long("lines")
        .default_value("10"),
    )
    .arg(
        Arg::new("bytes")
        .value_name("BYTES")
        .help("Number of bytes")
        .short('c')
        .takes_value(true)
        .conflicts_with("lines")
        .long("bytes"),
    )
    .get_matches();


    let lines = matches
    .value_of("lines")
    .map(parse_positive_int)
    .transpose()
    .map_err(|e| format!("illegal line count -- {}", e))?;

    let bytes = matches
    .value_of("bytes")
    .map(parse_positive_int)
    .transpose()
    .map_err(|e| format!("illegal byte count -- {}", e))?;

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        lines: lines.unwrap(),
        bytes
    })
}
fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn run(config: Config) -> MyResult<()> {
    for (file_num,filename) in config.files.iter().enumerate() {
        match open(&filename) {
            Err(e) => eprintln!("{}: {}", filename, e),
            Ok(mut file) => {
                if config.files.len() > 1 {
                        println!("{}==> {} <==", if file_num > 0 {"\n"} else {""}, filename);
                }
                if let Some(num_bytes) = config.bytes {
                    let mut buffer = vec!(0; num_bytes);
                    let mut handle = file.take(num_bytes as u64);
                    let read = handle.read(&mut buffer)?;
                    let s = String::from_utf8_lossy(&buffer[..read]);
                    print!("{}", s);
                } else {
                    let mut line = String::new();
                    // while let Ok(n) = file.read_line(&mut line) {
                    //     if n == 0 { break; } // eof
                    //     print!("{}", line);
                    //     line.clear();
                    // }
                    for _ in 0..config.lines {
                        let bytes = file.read_line(&mut line)?;
                        if bytes == 0 {break;}  // eof
                        print!("{}", line);
                        line.clear() // otherwise the data will accumulate in your buffer
                    }
                    }
        }
    }
}
    Ok(())
}

fn parse_positive_int(val: &str) -> MyResult<usize> {
    match val.parse() {
        Ok(n) if n > 0 => Ok(n),
        _ => Err(From::from(val)),
      }
}
#[test]
fn test_parse_positive_int() {
    // 3 is an OK int
    let res = parse_positive_int("3");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 3);

    // Any string is an error
    let res = parse_positive_int("foo");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "foo".to_string());

    // // Zero is an error
    // let res = parse_positive_int("0");
    // assert!(res.is_err());
    // assert_eq!(res.unwrap_err().to_string(), "0".to_string())
}