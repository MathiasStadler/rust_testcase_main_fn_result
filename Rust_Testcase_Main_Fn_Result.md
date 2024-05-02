# Explore Rust TestCase for cli main function,  fn, strut and result

[Update the local rust/cargo installation to the newest/latest version](https://github.com/MathiasStadler/repo_template/blob/main/includes/local_update_rust_env.md)

<details>
    <summary>Update local rust/cargo env to newest stand</summary>

## project setup for easy to used

### update rust of latest stable version

```bash
rustup update
```

### update all rust extension

#### install cargo-update

```bash
cargo install cargo update
```

#### Self-update of cargo-update

- cargo-update will update itself seamlessly on Linux and Windows.

```bash
cargo update
```

#### update all extension

```bash
cargo install update -- -a
```

- error: there is nothing to install in `update v0.0.0`, because it has no binaries
Where there is nothing, nothing can be updated

### check all necessary plugins installed

```bash
ls -la  ~/.cargo/bin/
```

### check cargo-edit

#### check is already installed and with which version

```bash
cargo install --list |grep cargo-edit
```

#### install cargo install /w command cargo add

```bash
cargo install cargo-edit
```

#### update any packages of project

```bash
cargo update
```

## rust switch from stable to nightly and back [found here](https://stackoverflow.com/questions/58226545/how-to-switch-between-rust-toolchains)

### find out which version is on start

```bash
rustc --version
```

### switch to nightly

```bash
rustup override set nightly
```

### switch to stable

```bash
rustup override set stable
```

</details>

## we use crate assert_cmd

- we would to use [assert_cmd](https://crates.io/crates/assert_cmd)
-- modified this example - [first testcase](https://github.com/assert-rs/assert_cmd/blob/master/tests/cargo.rs)

### add assert_cmd crate to project

```bash
cargo add assert_cmd
```

## simplest testcase - hello world

```rust
#!/usr/bin/env bash
export SCRIPT_FILE="01_testcase_hello_world.rs"
export SCRIPT_DIR="examples/"
cat << EoF > ./$SCRIPT_DIR/$SCRIPT_FILE
#[allow(unused_imports)]
use assert_cmd::Command;

fn main() {
    println!("Hello, world!");
}

#[test]
fn cargo_binary() {
    let mut cmd = Command::cargo_bin("$(echo $SCRIPT_FILE | cut -d . -f 1)").unwrap();
    cmd.assert().success().stdout("Hello, world!\n");
}

/*
export FILE_NAME=$SCRIPT_FILE
export FILE_DIR_NAME=$SCRIPT_DIR
git add \$FILE_DIR_NAME/\$FILE_NAME
git commit --all --message="-> Add BEFORE housekeeping => \$FILE_DIR_NAME/\$FILE_NAME"
# git push
# cargo install --list
# cargo update --workspace
cargo clippy --fix
cargo clippy --fix --examples
# cargo check --verbose
# cargo check --verbose --examples
cargo check
cargo check --examples
cargo fmt -- --emit=files \$FILE_DIR_NAME/\$FILE_NAME
git commit --all --message="-> Add AFTER housekeeping => \$FILE_DIR_NAME/\$FILE_NAME"
git push
echo "";
echo "run rust PRG => \$(echo \$FILE_NAME | cut -d . -f 1)";
cargo run --example "\$(echo \$FILE_NAME | cut -d . -f 1)"
echo "";
echo "run rust TEST => \$(echo \$FILE_NAME | cut -d . -f 1)"
cargo test --example "\$(echo \$FILE_NAME | cut -d . -f 1)"
echo "";
echo "ReturnCode => \$?"
*/

EoF
```

## testcase stdout and return_code

```rust
#!/usr/bin/env bash
export SCRIPT_FILE="02_testcase_stdin_stdout_return_code.rs"
export SCRIPT_DIR="examples/"
cat << EoF > ./$SCRIPT_DIR/$SCRIPT_FILE
// FROM HERE
// https://doc.rust-lang.org/std/io/struct.Stdin.html

// stdin
// https://docs.rs/assert_cmd/latest/assert_cmd/
#[allow(unused_imports)]
use assert_cmd::Command;

fn main() {
   let mut input = String::new();
match std::io::stdin().read_line(&mut input) {
    Ok(n) => {
        println!("{n} bytes read");
        println!("{input}");
    }
    Err(error) => println!("error: {error}"),
}
}

// stdin testcase
// https://docs.rs/assert_cmd/latest/assert_cmd/
#[test]
fn cargo_binary() {
    let mut cmd = Command::cargo_bin("$(echo $SCRIPT_FILE | cut -d . -f 1)").unwrap();
    // cmd.assert().success().stdout("Hello, world!\n");
    let assert = cmd
    .write_stdin("standard_in_str")
    .assert();
    assert
    .success()
    .code(0)
    .stdout("standard_in_str\n");
}

/*
export FILE_NAME=$SCRIPT_FILE
export FILE_DIR_NAME=$SCRIPT_DIR
git add \$FILE_DIR_NAME/\$FILE_NAME
git commit --all --message="-> Add BEFORE housekeeping => \$FILE_DIR_NAME/\$FILE_NAME"
# git push
# cargo install --list
# cargo update --workspace
cargo clippy --fix
cargo clippy --fix --examples
# cargo check --verbose
# cargo check --verbose --examples
cargo check
cargo check --examples
cargo fmt -- --emit=files \$FILE_DIR_NAME/\$FILE_NAME
git commit --all --message="-> Add AFTER housekeeping => \$FILE_DIR_NAME/\$FILE_NAME"
git push
echo "";
echo "run rust PRG => \$(echo \$FILE_NAME | cut -d . -f 1)";
cargo run --example "\$(echo \$FILE_NAME | cut -d . -f 1)"
echo "";
# DISABLE =>  because this testcase need keyboard input
# echo "run rust TEST => \$(echo \$FILE_NAME | cut -d . -f 1)"
cargo test --example "\$(echo \$FILE_NAME | cut -d . -f 1)"
echo "";
echo "ReturnCode => \$?"
*/

EoF
```

## testcase main run

```rust
#!/usr/bin/env bash
export SCRIPT_FILE="03_testcase_main_run.rs"
export SCRIPT_DIR="examples/"
cat << EoF > ./$SCRIPT_DIR/$SCRIPT_FILE
// FROM HERE
// https://raw.githubusercontent.com/assert-rs/assert_cmd/master/examples/example_fixture.rs

#![allow(clippy::exit)]

use std::env;
use std::error::Error;
use std::io;
use std::io::Write;
use std::process;

fn run() -> Result<(), Box<dyn Error>> {
    if let Ok(text) = env::var("stdout") {
        println!("{}", text);
    }
    if let Ok(text) = env::var("stderr") {
        eprintln!("{}", text);
    }

    let code = env::var("exit")
        .ok()
        .map(|v| v.parse::<i32>())
        .map(|r| r.map(Some))
        .unwrap_or(Ok(None))?
        .unwrap_or(0);
    process::exit(code);
}

fn main() {
    let code = match run() {
        Ok(_) => 0,
        Err(ref e) => {
            write!(&mut io::stderr(), "{}", e).expect("writing to stderr won't fail");
            1
        }
    };
    process::exit(code);
}

#[test]
fn cargo_binary() {
    let mut cmd = Command::cargo_bin("$(echo $SCRIPT_FILE | cut -d . -f 1)").unwrap();
    cmd.assert().success().stdout("Hello, world!\n");
}

/*
export FILE_NAME=$SCRIPT_FILE
export FILE_DIR_NAME=$SCRIPT_DIR
git add \$FILE_DIR_NAME/\$FILE_NAME
git commit --all --message="-> Add BEFORE housekeeping => \$FILE_DIR_NAME/\$FILE_NAME"
# git push
# cargo install --list
# cargo update --workspace
cargo clippy --fix
cargo clippy --fix --examples
# cargo check --verbose
# cargo check --verbose --examples
cargo check
cargo check --examples
cargo fmt -- --emit=files \$FILE_DIR_NAME/\$FILE_NAME
git commit --all --message="-> Add AFTER housekeeping => \$FILE_DIR_NAME/\$FILE_NAME"
git push
echo "";
echo "run rust PRG => \$(echo \$FILE_NAME | cut -d . -f 1)";
cargo run --example "\$(echo \$FILE_NAME | cut -d . -f 1)"
echo "";
echo "run rust TEST => \$(echo \$FILE_NAME | cut -d . -f 1)"
cargo test --example "\$(echo \$FILE_NAME | cut -d . -f 1)"
echo "";
echo "ReturnCode => \$?"
*/

EoF
```

## [next step](https://github.com/assert-rs/assert_cmd/blob/master/examples/example_fixture.rs)

[marker from here](https://github.com/MathiasStadler/repo_template/blob/main/includes/markdown_marker.md#to-highlight-a-note-and-warning-using-blockquote)
