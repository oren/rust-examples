fn parse_markdown_file() {
    // This will be created in Chapter 4
}

fn get_title() -> String {
    // create a String from a literal string with String::from
    let mut the_title = String::from(env!("CARGO_PKG_NAME"));
    // append a &str to a String with the push_str method
    the_title.push_str(" (v");
    the_title.push_str(env!("CARGO_PKG_VERSION"));
    the_title.push_str("), ");
    the_title.push_str(env!("CARGO_PKG_DESCRIPTION"));
    return the_title;
}

fn print_short_banner() {
    println!("{}", get_title());
}

fn print_long_banner() {
    print_short_banner();
    println!("Written by: {}\nHomepage: {}\nUsage: tinymd <somefile>.md\n",
    env!("CARGO_PKG_AUTHORS"),
    env!("CARGO_PKG_HOMEPAGE")
  );

}

fn usage() {
    print_long_banner();
}

fn main() {
    usage();
}

// References
// https://doc.rust-lang.org/std/string/struct.String.html
