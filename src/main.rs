extern crate getopts;
extern crate git_gsub;

use std::env;
use getopts::Options;

#[cfg_attr(test, allow(dead_code))]

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} <from> <to> [target_files]", program);
    print!("{}", opts.usage(&brief));
}

fn print_version() {
    println!("{}", env!("CARGO_PKG_VERSION"));
}

fn main() {
    let program: String = env::args().nth(0).unwrap();
    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");
    opts.optflag("v", "version", "print version");
    let matches = match opts.parse(&args[1..]) {
        Ok(m)  => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    if matches.opt_present("v") {
        return print_version();
    } else if matches.opt_present("h") || args.len() <= 2 {
        return print_usage(&program, opts);
    }

    git_gsub::run(args);
}
