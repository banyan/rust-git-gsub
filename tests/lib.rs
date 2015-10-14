extern crate tempdir;
extern crate git_gsub;

use std::env;
use std::fs::File;
use std::path::Path;
use tempdir::TempDir;
use std::error::Error;
use std::io::prelude::*;
use std::process::Command;

fn temp_dir() -> TempDir {
    TempDir::new("git-gsub-test").unwrap()
}

#[test]
fn repalce_the_words_if_git_gsub_run() {
    let tmpdir = temp_dir();
    let tmp_path_str: &str = tmpdir.path().to_str().unwrap();
    let _ = env::set_current_dir(&tmp_path_str);

    let path = Path::new("foo.txt");
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, Error::description(&why)),
        Ok(file) => file,
    };

    file.write_all("foo".as_bytes()).ok().expect("failed to write");

    Command::new("git")
            .arg("init")
            .status()
            .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });

    Command::new("git")
            .args(&["add", "."])
            .status()
            .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });

    // FIXME how to write better...
    let args: Vec<String> = vec!["git_gsub".to_string(), "foo".to_string(), "bar".to_string()];
    git_gsub::run(args);

    let path = Path::new("foo.txt");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, Error::description(&why)),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, Error::description(&why)),
        Ok(_)    => assert_eq!(s, "bar")
    }
}
