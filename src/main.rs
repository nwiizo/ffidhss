use std::error::Error;
use std::io::prelude::*;
use std::process::{Command, Stdio};
use std::env;

static PANGRAM: &'static str =
"the quick brown fox jumped over the lazy dog\n";

fn main() {
    let args: Vec<String> = env::args().collect();
    let checkfile = &args[1];
    let process = match Command::new("wc")
                                .arg(checkfile)
                                .stdin(Stdio::piped())
                                .stdout(Stdio::piped())
                                .spawn() {
        Err(why) => panic!("couldn't spawn wc: {}", Error::description(&why)),
        Ok(process) => process,
    };

    match process.stdin.unwrap().write_all(PANGRAM.as_bytes()) {
        Err(why) => panic!("couldn't write to wc stdin: {}",
                           Error::description(&why)),
        Ok(_) => println!("sent pangram to wc"),
    }

    let mut s = String::new();
    match process.stdout.unwrap().read_to_string(&mut s) {
        Err(why) => panic!("couldn't read wc stdout: {}",
                           Error::description(&why)),
        Ok(_) => print!("wc responded with:\n{}", s),
    }
}
