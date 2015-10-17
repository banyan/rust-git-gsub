# rust-git-gsub [![Circle CI](https://img.shields.io/circleci/project/banyan/rust-git-gsub.svg)](https://circleci.com/gh/banyan/rust-git-gsub)

>A Git subcommand to do gsub in a repository.

Ported from [fujimura/git-gsub](https://github.com/fujimura/git-gsub).

## Usage

To substitute `Git` with `Subversion`, run

```
$ git gsub Git Subversion
```

Then you will get

```diff
diff --git a/README.md b/README.md
index 2185dbf..393dbc6 100644
--- a/README.md
+++ b/README.md
@@ -1,4 +1,4 @@
-# Git::Gsub
+# Subversion::Gsub

 TODO: Write a gem description

diff --git a/bin/git-gsub b/bin/git-gsub
index c30f093..03b7c4c 100755
--- a/bin/git-gsub
+++ b/bin/git-gsub
@@ -1,4 +1,4 @@
 #! /usr/bin/env ruby

 require 'git/gsub'
-Git::Gsub.run
+Subversion::Gsub.run
```

## Installation

```zsh
# Install Rust > 1.3.0
git clone git@github.com:banyan/rust-git-gsub.git && cd rust-git-gsub
cargo build --release
cp target/release/git-gsub /somewhere/in/your/$PATH
```

## License

MIT
