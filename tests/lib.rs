extern crate tempdir;
extern crate git_gsub;

use std::env;
use std::fs::File;
use std::path::Path;
use tempdir::TempDir;
use std::io::prelude::*;
use std::process::Command;

fn assert_substitution(from: &str, to: &str) {
    let tmpdir = TempDir::new("git-gsub-test").unwrap();
    let tmp_path_str: &str = tmpdir.path().to_str().unwrap();
    env::set_current_dir(&tmp_path_str).unwrap();
    let path = Path::new("foo.txt");
    let mut file = File::create(&path).unwrap();
    file.write_all("foo".as_bytes()).unwrap();
    Command::new("git")
            .arg("init")
            .status()
            .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
    Command::new("git")
            .args(&["add", "."])
            .status()
            .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
    let args = vec!["git_gsub", from, to].iter().map(|c| c.to_string()).collect::<Vec<String>>();
    git_gsub::substitute(args);
    let mut file = File::open(&path).unwrap();
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();
    println!("{:?}", s);
    assert_eq!(s, to);
}

#[test]
fn it_substitutes() {
    assert_substitution("foo", "bar");
}

#[test]
fn it_escapes_well() {
    assert_substitution("<h1 class=\"foo\">", "<h1 class=\"bar\">");
}
