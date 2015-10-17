extern crate shlex;
extern crate getopts;

use std::env;
use getopts::Options;
use std::process::exit;
use std::process::Command;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} <from> <to> [target_files]", program);
    print!("{}", opts.usage(&brief));
}

fn print_version() {
    println!("{}", env!("CARGO_PKG_VERSION"));
}

fn is_gsed_installed() -> bool {
    Command::new("which")
            .arg("gsed")
            .output()
            .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) })
            .status
            .success()
}

fn quote_args(args: &Vec<String>) -> (String, String, String) {
    let quoted_args: Vec<String> = args[1..].iter().map(|x| shlex::quote(x).to_string() ).collect();
    (
        quoted_args[0].clone(),
        quoted_args[1].clone(),
        if quoted_args.len() > 2 { quoted_args[2].clone() } else { ".".to_string() }
    )
}

pub fn substitute(args: &Vec<String>) -> () {
    let (from, to, path) = quote_args(&args);

    let output = Command::new("git")
                         .args(&["grep", "-l"])
                         .arg(&from)
                         .arg(&path)
                         .output()
                         .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });

    let stdout = String::from_utf8_lossy(&output.stdout);
    let target_files: Vec<&str> = stdout.lines_any().collect();
    if target_files.len() == 0 { exit(0); }
    let re = format!("s/{}/{}/g", &from, &to);

    if is_gsed_installed() {
        Command::new("gsed")
                .arg("-i")
                .arg(&re)
                .args(&target_files)
                .status()
                .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
    } else {
        Command::new("sed")
                .args(&["-i", "", "-e"])
                .arg(&re)
                .args(&target_files)
                .status()
                .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
    }
}

pub fn run(args: env::Args) -> () {
    let args: Vec<String> = args.collect();
    let ref program = args[0];

    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");
    opts.optflag("v", "version", "print version");

    if args.len() <= 2 { return print_usage(&program, opts) }

    let matches = match opts.parse(&args[1..]) {
        Ok(m)  => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    if matches.opt_present("v") {
        return print_version();
    } else if matches.opt_present("h") {
        return print_usage(&program, opts);
    }

    substitute(&args);
}
