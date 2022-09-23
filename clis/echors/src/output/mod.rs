pub fn print_omit_text(text: Vec<String>, omit_newline: bool) -> () {
    print!("{}{}", text.join(" "), if omit_newline { "" } else { "\n" });
}
