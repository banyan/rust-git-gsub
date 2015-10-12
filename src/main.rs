extern crate getopts;
extern crate shlex;

use getopts::Options;
use std::env;

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

    let quoted_args: Vec<String> = args[1..]
        .iter()
        .map(|x| shlex::quote(x).to_string() )
        .collect();

    let from = quoted_args[0].clone();
    let to   = quoted_args[1].clone();
    let path = if quoted_args.len() > 2 { quoted_args[2].clone() } else { ".".to_string() };

    let output = Command::new("git")
                         .arg("grep")
                         .arg("-l")
                         .arg(&from)
                         .arg(&path)
                         .output()
                         .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });

    let target_files_with_line_break = String::from_utf8_lossy(&output.stdout);
    let target_files: Vec<&str> = target_files_with_line_break.lines_any().collect();
    if target_files.len() == 0 { return; }
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
                .arg("-i")
                .arg("")
                .arg("-e")
                .arg(&re)
                .args(&target_files)
                .status()
                .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });

    }
}
