extern crate git_gsub;

use std::env;

#[cfg_attr(test, allow(dead_code))]
fn main() {
    git_gsub::run(env::args());
}
