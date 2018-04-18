use std::error::Error;
use std::io::prelude::*;
use std::process::{Command, Stdio};
use std::env;

static PANGRAM: &'static str = "the quick brown fox jumped over the lazy dog\n";

fn main() {
    let args: Vec<String> = env::args().collect();
    let checkfile = &args[1];
    let remotefile = &args[2];
    let ruser = &args[3];
    let rserver = &args[4];
    let ssh_file = format!("(ssh {}@{} 'cat {}')",ruser,rserver,remotefile);
    let process = match Command::new("diff")
                                .arg("<")
                                .arg(ssh_file)
                                .arg(checkfile)
                                //.arg(remotefile)
                                .stdin(Stdio::piped())
                                .stdout(Stdio::piped())
                                .spawn() {
        Err(why) => panic!("couldn't spawn diff: {}", Error::description(&why)),
        Ok(process) => process,
    };

    match process.stdin.unwrap().write_all(PANGRAM.as_bytes()) {
        Err(why) => panic!("couldn't write to diff stdin: {}",
                           Error::description(&why)),
        Ok(_) => println!("sent pangram to diff"),
    }

    let mut s = String::new();
    match process.stdout.unwrap().read_to_string(&mut s) {
        Err(why) => panic!("couldn't read diff stdout: {}",
                           Error::description(&why)),
        Ok(_) => print!("diff responded with:\n{}", s),
    }
}
