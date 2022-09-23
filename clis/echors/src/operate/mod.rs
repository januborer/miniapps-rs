use clap::ArgMatches;

pub fn get_omit_text(matches: ArgMatches) -> (Vec<String>, bool) {
    (
        matches.values_of_lossy("text").unwrap(),
        matches.is_present("omit_newline"),
    )
}
