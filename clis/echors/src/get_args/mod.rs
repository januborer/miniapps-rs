use clap::{App, Arg, ArgMatches};
pub fn get_args() -> ArgMatches<'static> {
    App::new("echors")
        .version("0.1.0")
        .author("januborer <janu.yieg.borer@gmail.com>")
        .about("Rust echo")
        .arg(
            Arg::with_name("text")
                .value_name("TEXT")
                .help("Input text")
                .required(true)
                .min_values(1),
        )
        .arg(
            Arg::with_name("omit_newline")
                .short("n")
                .help("Do not print newline")
                .takes_value(false),
        )
        .get_matches()
}
