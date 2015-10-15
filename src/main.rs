extern crate git_gsub;

#[cfg_attr(test, allow(dead_code))]
fn main() {
    git_gsub::run(std::env::args());
}
