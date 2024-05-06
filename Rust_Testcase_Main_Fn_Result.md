# Explore Rust TestCase for cli main function,  fn, strut and result

<details>
<summary>Usefully thing</summary>

[Update the local rust/cargo installation to the newest/latest version](https://github.com/MathiasStadler/repo_template/blob/main/includes/local_update_rust_env.md)

[Extract all rust code block from markdown file](https://github.com/MathiasStadler/repo_template/blob/main/includes/extract__scripts_from_markdown.md)

[Markdown marker](https://github.com/MathiasStadler/repo_template/blob/main/includes/markdown_marker.md#to-highlight-a-note-and-warning-using-blockquote)

[Markdown template rust codeblock](https://github.com/MathiasStadler/repo_template/blob/main/includes/dummy_rust_codeblock.md)

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

## testcase stdout

```rust
#!/usr/bin/env bash
export SCRIPT_FILE="02_testcase_stdin_stdout.rs"
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
        #[allow(unused_variables)]
        Ok(n) => {
            // println!("{n} bytes read");
            println!("{input}");
        }
        Err(error) => println!("error: {error}"),
    }
}

// stdin testcase
// https://docs.rs/assert_cmd/latest/assert_cmd/
#[test]
fn testcase_stdin_stdout_success() {
    let mut cmd = Command::cargo_bin("$(echo $SCRIPT_FILE | cut -d . -f 1)").unwrap();
    // cmd.assert().success().stdout("Hello, world!\n");
    let assert = cmd
    .write_stdin("std_in_str")
    .assert();
    assert
    .success()
    .code(0)
    .stdout("std_in_str\n");
}

#[test]
fn testcase_stdin_stdout_failure() {
    let mut cmd = Command::cargo_bin("$(echo $SCRIPT_FILE | cut -d . -f 1)").unwrap();
    let assert = cmd
    .write_stdin("std_in_str")
    .assert();
    assert
    .failure()
    .code(0)
    .stdout("failed_std_in_str\n");
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
# **DISABLE BY HAND** because this program use manually input from user
# echo "run rust PRG => \$(echo \$FILE_NAME | cut -d . -f 1)";
# cargo run --example "\$(echo \$FILE_NAME | cut -d . -f 1)"
echo "";
# instead make only a build
cargo build --example "\$(echo \$FILE_NAME | cut -d . -f 1)"
echo "";
# DISABLE =>  because this testcase need keyboard input
# echo "run rust TEST => \$(echo \$FILE_NAME | cut -d . -f 1)"
cargo test --example "\$(echo \$FILE_NAME | cut -d . -f 1)"
echo "";
echo "ReturnCode => \$?"
*/

/* run oncommand line
cargo test --package rust_testcase_main_fn_result --example \$(echo \$FILE_NAME | cut -d . -f 1) --  --exact --show-output --nocapture
*/

EoF
```

## testcase return_code

> [FROM HERE](https://stackoverflow.com/questions/43390971/how-to-check-the-exit-code-from-stdprocessexit-in-tests)

```rust
#!/usr/bin/env bash
export SCRIPT_FILE="03_testcase_return_code.rs"
export SCRIPT_DIR="examples/"
cat << EoF > ./$SCRIPT_DIR/$SCRIPT_FILE
// First hint/approach
//https://doc.rust-lang.org/std/process/struct.ExitCode.html
//fn main() -> ExitCode {

//// First hint/approach
// Executes the command as a child process, waiting for it to finish and collecting all of its output.
// https://doc.rust-lang.org/std/process/struct.Command.html#method.output
#[allow(unused_imports)]
use std::process::Command;
#[allow(unused_imports)]
use std::io::{self, Write};
#[allow(unused_imports)]
use std::process::ExitStatus;
#[allow(unused_imports)]
use std::process;

fn main(){

println!("Hello example!");
process::exit(0);
    
}

#[test]
fn test_main_return_code_succes(){
    let output = Command::new("/bin/cat")
    .arg("/etc/os-release")
    .output()
    .expect("failed to execute process");

println!("status: {}", output.status);
assert_eq!(<ExitStatus as std::default::Default>::default(), output.status);
io::stdout().write_all(&output.stdout).unwrap();
io::stderr().write_all(&output.stderr).unwrap();

}

#[test]
fn test_main_return_code_failure(){
    let output = Command::new("/bin/cat")
    .arg("/tmp/not_available_file.txt")
    .output()
    .expect("failed to execute process");

println!("status: {}", output.status);
assert_eq!(<ExitStatus as std::default::Default>::default(), output.status);
io::stdout().write_all(&output.stdout).unwrap();
io::stderr().write_all(&output.stderr).unwrap();

}

/*
export FILE_NAME=03_testcase_return_code.rs
export FILE_DIR_NAME=examples/
git add $FILE_DIR_NAME/$FILE_NAME
git commit --all --message="-> Add BEFORE housekeeping => $FILE_DIR_NAME/$FILE_NAME"
# git push
# cargo install --list
# cargo update --workspace
cargo clippy --fix
cargo clippy --fix --examples
cargo  clippy --fix --examples --all-features
cargo  clippy --fix --examples --all-features  -- -Dwarnings
# cargo check --verbose
# cargo check --verbose --examples
cargo check
cargo check --examples
cargo fmt -- --emit=files $FILE_DIR_NAME/$FILE_NAME
git commit --all --message="-> Add AFTER housekeeping => $FILE_DIR_NAME/$FILE_NAME"
git push
echo "";
# **DISABLE BY HAND** because this program use manually input from user
# echo "run rust PRG => $(echo $FILE_NAME | cut -d . -f 1)";
# cargo run --example "$(echo $FILE_NAME | cut -d . -f 1)"
echo "";
# instead make only a build
cargo build --example "$(echo $FILE_NAME | cut -d . -f 1)"
echo "";
# DISABLE =>  because this testcase need keyboard input
# echo "run rust TEST => $(echo $FILE_NAME | cut -d . -f 1)"
cargo test --example "$(echo $FILE_NAME | cut -d . -f 1)"
echo "";
echo "ReturnCode => $?"
*/

/* run oncommand line
cargo test --package rust_testcase_main_fn_result --example $(echo $FILE_NAME | cut -d . -f 1) --  --exact --show-output --nocapture
*/

EoF
```

## testcase main run

```rust
#!/usr/bin/env bash
export SCRIPT_FILE="04_testcase_main_run.rs"
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

[Markdown marker from here](https://github.com/MathiasStadler/repo_template/blob/main/includes/markdown_marker.md#to-highlight-a-note-and-warning-using-blockquote)
