use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::process::{Command, Stdio};

extern crate regex;
use regex::Regex;

macro_rules! fail {
    ($($t:tt)*) => {{
        eprintln!($($t)*);
        std::process::exit(1);
    }};
}

fn need(arg: Option<String>, desc: &str, default: Option<String>) -> String {
    match arg {
        None => match default {
            None => fail!("Need argument '{desc}'!"),
            Some(default) => default,
        },
        Some(arg) => arg,
    }
}

fn main() {
    let mut args = env::args();
    let _programm = args.next();

    // Open input file.
    let filename = need(args.next(), "file name", None);
    let path = Path::new(&filename);
    let file = match File::open(path) {
        Err(why) => fail!("Can not open file '{filename}': {why}"),
        Ok(file) => file,
    };
    let mut reader = BufReader::new(file);

    // Create regular expressions for parsing.
    let language = need(args.next(), "language", None);
    let begin_rx_string = format!("```{language}");
    let begin_code = match Regex::new(&begin_rx_string) {
        Err(why) => fail!("Can not compile regex '{begin_rx_string}': {why}"),
        Ok(rx) => rx,
    };
    let end_code = match Regex::new(r"```") {
        Err(why) => fail!("Can not compile regex '{begin_rx_string}': {why}"),
        Ok(rx) => rx,
    };

    // Start script program.
    let executable = need(args.next(), "executable", Some(language.clone()));
    let mut process = match Command::new(&executable).stdin(Stdio::piped()).spawn() {
        Err(why) => fail!("Can not execute script program '{executable}': {why}"),
        Ok(process) => process,
    };
    let mut pipe = match process.stdin {
        None => fail!("Can not get stdin of script program '{executable}'."),
        Some(ref pipe) => pipe,
    };

    // Parse input file and send data to script program.
    let mut within_code = false;
    let mut line = String::new();
    loop {
        line.clear();
        let bytes = match reader.read_line(&mut line) {
            Err(why) => fail!("Can not read line: {why}"),
            Ok(bytes) => bytes,
        };
        if bytes == 0 {
            break;
        }
        if !within_code {
            if begin_code.is_match(&line) {
                within_code = true;
            }
        } else {
            if end_code.is_match(&line) {
                within_code = false;
            } else {
                match pipe.write_all(line.as_bytes()) {
                    Err(why) => fail!("Can not write to script program: {why}"),
                    Ok(_) => (),
                };
            }
        }
    }

    // Wait for child.
    match process.wait() {
        Err(why) => fail!("Can not wait for script program: {why}"),
        Ok(_) => (),
    };
}

// Local Variables:
// compile-command: "cargo build"
// End:
