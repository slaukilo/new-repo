#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

mod util;

use clap::{Arg, ArgMatches, Command};
use std::process;
fn main() {
    let matches = Command::new("rtch")
        .version("1.0")
        .author("Mortiferis <me@email.com>")
        .about("rtch is a lightweight and simple fetch system built \
                in Rust!")
        .after_help("Longer explanation to appear after the options when \
                displaying the help information from --help or -h")
        .display_name("rtch")
        .get_matches();
   
}
