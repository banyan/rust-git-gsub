extern crate shlex;

use std::process::Command;

fn is_gsed_installed() -> bool {
    Command::new("which")
            .arg("gsed")
            .output()
            .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) })
            .status
            .success()
}

fn quote_args(args: Vec<String>) -> (String, String, String) {
    let quoted_args: Vec<String> = args[1..].iter().map(|x| shlex::quote(x).to_string() ).collect();
    (
        quoted_args[0].clone(),
        quoted_args[1].clone(),
        if quoted_args.len() > 2 { quoted_args[2].clone() } else { ".".to_string() }
    )
}

pub fn run(args: Vec<String>) -> () {
    let (from, to, path) = quote_args(args);

    let output = Command::new("git")
                         .args(&["grep", "-l"])
                         .arg(&from)
                         .arg(&path)
                         .output()
                         .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });

    let stdout = String::from_utf8_lossy(&output.stdout);
    let target_files: Vec<&str> = stdout.lines_any().collect();
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
                .args(&["-i", "", "-e"])
                .arg(&re)
                .args(&target_files)
                .status()
                .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });

    }
}
