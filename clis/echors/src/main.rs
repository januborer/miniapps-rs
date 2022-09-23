mod get_args;
mod operate;
mod output;
fn main() {
    let matches = get_args::get_args();

    let (text, omit_newline) = operate::get_omit_text(matches);

    output::print_omit_text(text, omit_newline);
}
