use clap::{App, Arg};

fn main() {
    let matches = App::new("echor")
        .version("0.1.1")
        .author("Bill Hegazy <bill.hegazy@gmail.com>")
        .about("Rust Echo")
        .arg(
            Arg::with_name("text")
                .value_name("TEXT")
                .help("Input text")
                .required(true)
                .min_values(1),
        )
        .arg(
            Arg::with_name("omit_newline")
                .help("Do not print new line")
                .short("n")
                .takes_value(false),
        )
        .get_matches();
    let text = matches.values_of_lossy("text").unwrap();
    let omit_newline = matches.is_present("omit_newline");

    // let ending = if omit_newline { "" } else { "\n" };

    print!("{}{}", text.join(" "), if omit_newline { "" } else { "\n" });
}
