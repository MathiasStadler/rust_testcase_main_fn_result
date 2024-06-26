# Explore Rust TestCase for cli main function, fn, strut and result

<details>
<summary>Usefully thing</summary>

[Update the local rust/cargo installation to the newest/latest version](https://github.com/MathiasStadler/repo_template/blob/main/includes/local_update_rust_env.md)

[Extract all code block from markdown with rust](https://github.com/MathiasStadler/repo_template/blob/main/includes/extract__scripts_from_markdown.md)

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
export SCRIPT_PACKAGE="rust_testcase_main_fn_result";
export SCRIPT_FILE="01_testcase_hello_world.rs";
export SCRIPT_DIR="examples/";
# make dir if not available
mkdir -p ./$SCRIPT_DIR;
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
export SCRIPT_PACKAGE=$SCRIPT_PACKAGE
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
echo "ReturnCode => \$?"
*/

/*
# quick => run testcase from shell command prompt
cargo test --package "$SCRIPT_PACKAGE" --example "$(echo $SCRIPT_FILE | cut -d . -f 1)" --  --exact --show-output --nocapture
*/

EoF
```

## testcase stdout

```rust
#!/usr/bin/env bash
export SCRIPT_PACKAGE="rust_testcase_main_fn_result";
export SCRIPT_FILE="02_testcase_stdin_stdout.rs";
export SCRIPT_DIR="examples/";
# make dir if not available
mkdir -p ./$SCRIPT_DIR;
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
#[should_panic(expected = "Unexpected success")]
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
export SCRIPT_PACKAGE=$SCRIPT_PACKAGE
export FILE_NAME=$SCRIPT_FILE
export FILE_DIR_NAME=$SCRIPT_DIR
git add \$FILE_DIR_NAME/\$FILE_NAME
git commit --all --message="-> Add BEFORE housekeeping => \$FILE_DIR_NAME/\$FILE_NAME"
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
echo "ReturnCode => \$?"
*/

/*
# quick => run testcase from shell command prompt
cargo test --package "$SCRIPT_PACKAGE" --example "$(echo $SCRIPT_FILE | cut -d . -f 1)" --  --exact --show-output --nocapture
*/

EoF
```

## testcase return_code

> [FROM HERE](https://stackoverflow.com/questions/43390971/how-to-check-the-exit-code-from-stdprocessexit-in-tests)

```rust
#!/usr/bin/env bash
export SCRIPT_PACKAGE="rust_testcase_main_fn_result";
export SCRIPT_FILE="03_testcase_return_code.rs";
export SCRIPT_DIR="examples/";
# make dir if not available
mkdir -p ./$SCRIPT_DIR;
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
fn test_main_return_code_success(){
    let output = Command::new("/bin/cat")
    .arg("/etc/os-release")
    .output()
    .expect("failed to execute process");

println!("status: {}", output.status);
assert_eq!(<ExitStatus as std::default::Default>::default(), output.status);
io::stdout().write_all(&output.stdout).unwrap();
io::stderr().write_all(&output.stderr).unwrap();

}
// FROM HERE - testcase - - should panic
// https://doc.rust-lang.org/rust-by-example/testing/unit_testing.html#testing-panics
#[test]
#[should_panic(expected = "ExitStatus(unix_wait_status(256))")]
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
export FILE_NAME=$SCRIPT_FILE
export FILE_DIR_NAME=$SCRIPT_DIR
export SCRIPT_PACKAGE=$SCRIPT_PACKAGE
git add \$FILE_DIR_NAME/\$FILE_NAME
git commit --all --message="-> Add BEFORE housekeeping => \$FILE_DIR_NAME/\$FILE_NAME"
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
echo "ReturnCode => \$?"
*/

/*
# quick => run testcase from shell command prompt
cargo test --package "$SCRIPT_PACKAGE" --example "$(echo $SCRIPT_FILE | cut -d . -f 1)" --  --exact --show-output --nocapture
*/

EoF
```

## testcase main run

```rust
#!/usr/bin/env bash
export SCRIPT_PACKAGE="rust_testcase_main_fn_result";
export SCRIPT_FILE="04_testcase_main_run.rs";
export SCRIPT_DIR="examples/";
# make dir if not available
mkdir -p ./$SCRIPT_DIR;
cat << EoF > ./$SCRIPT_DIR/$SCRIPT_FILE
// FROM HERE
// https://raw.githubusercontent.com/assert-rs/assert_cmd/master/examples/example_fixture.rs

#![allow(clippy::exit)]

use std::env;
use std::error::Error;
use std::io;
use std::io::Write;
use std::process;
#[allow(unused_imports)]
use assert_cmd::Command;

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

#[ignore]
#[test]
fn test_main_run() {
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
echo "ReturnCode => \$?"
*/

/*
# quick => run testcase from shell command prompt
cargo test --package "$SCRIPT_PACKAGE" --example "$(echo $SCRIPT_FILE | cut -d . -f 1)" --  --exact --show-output --nocapture
*/

EoF
```

## testcase fn result

- [FIRST HINT](https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/second-edition/ch11-01-writing-tests.html)
- [SECOND HINT](https://stackoverflow.com/questions/57234140/how-to-assert-io-errors-in-rust)

```rust
#!/usr/bin/env bash
export SCRIPT_PACKAGE="rust_testcase_main_fn_result";
export SCRIPT_FILE="05_testcase_fn_result.rs";
export SCRIPT_DIR="examples/";
# make dir if not available
mkdir -p ./$SCRIPT_DIR;
cat << EoF > ./$SCRIPT_DIR/$SCRIPT_FILE
// FORM HERE
// https://stackoverflow.com/questions/57234140/how-to-assert-io-errors-in-rust
use std::io;

fn main(){

println!("$SCRIPT_FILE");

}//end of main

#[allow(dead_code)]
fn parse_data(input: i32) -> Result<i32, io::Error> {
    match input {
        0 => Ok(0),
        1 => Ok(1),
        x => Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("unexpected number {}", x),
        )),
    }
}

#[test]
fn test_parsing_wrong_data_result_success() {
    let result = parse_data(0).map_err(|e| e.kind());
    let expected = Ok(0);
    assert_eq!(expected, result);
    }

#[test]
fn test_parsing_wrong_data_result_is_ok() {
    let result = parse_data(0).map_err(|e| e.kind());
    // instead
    //  let expected = Ok(0);
    //  assert_eq!(expected, result)
    // use that/this
    // is valid for all successful testcase
    assert!(result.is_ok());
}

#[test]
fn test_parsing_wrong_data_2nd_result_is_ok_ok() {
    // change parse data input to 1
    // otherwise identical testcase
    let result = parse_data(1).map_err(|e| e.kind());
    // instead
    //  let expected = Ok(0);
    //  assert_eq!(expected, result)
    // use that/this
    // is valid for all successful testcase
    assert!(result.is_ok());
}

#[test]
fn test_parsing_wrong_data_2nd_result_is_ok_failed() {
    // change parse data input to 1
    // otherwise identical testcase
    let result = parse_data(-1).map_err(|e| e.kind());
    // instead
    //  let expected = Ok(0);
    //  assert_eq!(expected, result)
    // use that/this
    // is valid for all successful testcase
    assert!(result.is_ok());
}

#[test]
fn test_parsing_wrong_data_of_failures_give_ok() {
    let result = parse_data(-1).map_err(|e| e.kind());
    let expected = Err(io::ErrorKind::InvalidData);
    assert_eq!(expected, result);
}

#[test]
fn test_parsing_wrong_data_result_is_err_ok() {
    let result = parse_data(-1).map_err(|e| e.kind());
    // let expected = Err(io::ErrorKind::InvalidData);
    // assert_eq!(expected, result);
    assert!(result.is_err());
}

#[test]
fn test_parsing_wrong_data_result_is_err_failed() {
    let result = parse_data(0).map_err(|e| e.kind());
    // let expected = Err(io::ErrorKind::InvalidData);
    // assert_eq!(expected, result);
    assert!(result.is_err());
}


/*
export FILE_NAME=$SCRIPT_FILE
export FILE_DIR_NAME=$SCRIPT_DIR
echo "build prg => \$(echo \$FILE_NAME | cut -d . -f 1)";
cargo build --example "\$(echo \$FILE_NAME | cut -d . -f 1)"
echo "run PRG => \$(echo \$FILE_NAME | cut -d . -f 1)";
cargo run --example "\$(echo \$FILE_NAME | cut -d . -f 1)"
echo "";
echo "run TEST => \$(echo \$FILE_NAME | cut -d . -f 1)"
cargo test --example "\$(echo \$FILE_NAME | cut -d . -f 1)"
cargo test --jobs $(grep -c ^processor /proc/cpuinfo) --example "\$(echo \
\$FILE_NAME | cut -d . -f 1)"
echo "ReturnCode => \$?"
*/

/*
# quick => run testcase from shell command prompt
cargo test --package "$SCRIPT_PACKAGE" --example "$(echo $SCRIPT_FILE | cut -d . -f 1)" --  --exact --show-output --nocapture
*/

EoF
```

### 06_ok_testcase_most_simple_result_from_fn.rs

```rust,no_run
#!/usr/bin/env bash
export EXAMPLE_SCRIPT_FILE="06_ok_testcase_most_simple_result_from_fn.rs"
export EXAMPLE_SCRIPT_DIR="examples/"
cat << EoF > ./$EXAMPLE_SCRIPT_DIR/$EXAMPLE_SCRIPT_FILE
// FOUND HERE
// https://stackoverflow.com/questions/45583246/rust-returns-a-result-error-from-fn-mismatched-types
fn get_result_success() -> Result<String, String> {
    Ok(String::from("foo")) // <- works fine
    //Result::Err(String::from("Error"))
 }
 
 fn main(){
     match get_result_success(){
         Ok(s) => println!("{}",s),
         Err(s) => println!("{}",s)
     };
 }

#[test]
fn test_get_result_success() {
    let result = get_result_success();
    let expected = Ok("foo");
    // What’s difference between as_deref() and * to a &variable?
    // https://users.rust-lang.org/t/whats-difference-between-as-deref-and-to-a-variable/103692/2
    assert_eq!(expected, result.as_deref());
    }


 
 /*
export FILE_NAME=$SCRIPT_FILE
export FILE_DIR_NAME=$SCRIPT_DIR
echo "build prg => \$(echo \$FILE_NAME | cut -d . -f 1)";
cargo build --example "\$(echo \$FILE_NAME | cut -d . -f 1)"
echo "run PRG => \$(echo \$FILE_NAME | cut -d . -f 1)";
cargo run --example "\$(echo \$FILE_NAME | cut -d . -f 1)"
echo "";
echo "run TEST => \$(echo \$FILE_NAME | cut -d . -f 1)"
cargo test --example "\$(echo \$FILE_NAME | cut -d . -f 1)"
cargo test --jobs $(grep -c ^processor /proc/cpuinfo) --example "\$(echo \
\$FILE_NAME | cut -d . -f 1)"
echo "ReturnCode => \$?"
*/

/*
# quick => run testcase from shell command prompt
cargo test --package "$SCRIPT_PACKAGE" --example "$(echo $SCRIPT_FILE | cut -d . -f 1)" --  --exact --show-output --nocapture
*/

EoF
```

### 06_err_testcase_most_simple_result_from_fn.rs

```rust,no_run
#!/usr/bin/env bash
export EXAMPLE_SCRIPT_FILE="06_err_testcase_most_simple_result_from_fn.rs"
export EXAMPLE_SCRIPT_DIR="examples/"
cat << EoF > ./$EXAMPLE_SCRIPT_DIR/$EXAMPLE_SCRIPT_FILE
// FOUND HERE
// https://stackoverflow.com/questions/45583246/rust-returns-a-result-error-from-fn-mismatched-types
fn get_result_error() -> Result<String, String> {
    // Ok(String::from("foo")) // <- works fine
    Result::Err(String::from("Error"))
 }
 
 fn main(){
     match get_result_error(){
         Ok(s) => println!("{}",s),
         Err(s) => println!("{}",s)
     };
 }

#[test]
fn test_get_result_error() {
    let result = get_result_error();
    // let expected = Ok("foo");
    // let expected:&&'static str = &"Error";
    // let should:&&'static str = &"Error";
    // What’s difference between as_deref() and * to a &variable?
    // https://users.rust-lang.org/t/whats-difference-between-as-deref-and-to-a-variable/103692/2
    // assert_eq!(expected, result.as_deref());
    assert!(result.is_err());
    }


 
 /*
export FILE_NAME=$SCRIPT_FILE
export FILE_DIR_NAME=$SCRIPT_DIR
echo "build prg => \$(echo \$FILE_NAME | cut -d . -f 1)";
cargo build --example "\$(echo \$FILE_NAME | cut -d . -f 1)"
echo "run PRG => \$(echo \$FILE_NAME | cut -d . -f 1)";
cargo run --example "\$(echo \$FILE_NAME | cut -d . -f 1)"
echo "";
echo "run TEST => \$(echo \$FILE_NAME | cut -d . -f 1)"
cargo test --example "\$(echo \$FILE_NAME | cut -d . -f 1)"
cargo test --jobs $(grep -c ^processor /proc/cpuinfo) --example "\$(echo \
\$FILE_NAME | cut -d . -f 1)"
echo "ReturnCode => \$?"
*/

/*
# quick => run testcase from shell command prompt
cargo test --package "$SCRIPT_PACKAGE" --example "$(echo $SCRIPT_FILE | cut -d . -f 1)" --  --exact --show-output --nocapture
*/

EoF
```

### std::result

[FROM HERE](https://doc.rust-lang.org/std/result/)

[no_run](https://doc.rust-lang.org/rustdoc/write-documentation/documentation-tests.html#attributes)

```rust,no_run
#!/usr/bin/env bash
export EXAMPLE_SCRIPT_FILE="07_rust_std_result.rs"
export EXAMPLE_SCRIPT_DIR="examples/"
cat << EoF > ./$EXAMPLE_SCRIPT_DIR/$EXAMPLE_SCRIPT_FILE
//FROM HERE
//https://doc.rust-lang.org/std/result/

#[derive(Debug)]
enum Version { Version1, Version2 }

fn parse_version(header: &[u8]) -> Result<Version, &'static str> {
    match header.get(0) {
        None => Err("invalid header length"),
        Some(&1) => Ok(Version::Version1),
        Some(&2) => Ok(Version::Version2),
        Some(_) => Err("invalid version"),
    }
}

pub fn main(){
    let version = parse_version(&[1, 2, 3, 4]);
    match version {
        Ok(v) => println!("working with version: {v:?}"),
        Err(e) => println!("error parsing header: {e:?}"),
    }
    
}

/*
export FILE_NAME=$EXAMPLE_SCRIPT_FILE
export FILE_DIR_NAME=$EXAMPLE_SCRIPT_DIR
echo "build prg => \$(echo \$FILE_NAME | cut -d . -f 1)";
cargo build --example "\$(echo \$FILE_NAME | cut -d . -f 1)"
echo "run PRG => \$(echo \$FILE_NAME | cut -d . -f 1)";
cargo run --example "\$(echo \$FILE_NAME | cut -d . -f 1)"
echo "";
echo "run TEST => \$(echo \$FILE_NAME | cut -d . -f 1)"
cargo test --example "\$(echo \$FILE_NAME | cut -d . -f 1)"
cargo test --jobs 4 --example "\$(echo \$FILE_NAME | cut -d . -f 1)"
echo "ReturnCode => $?"
*/
EoF

```

### rust result is_ok

```rust,no_run
#!/usr/bin/env bash
export EXAMPLE_SCRIPT_FILE="08_rust_result_is_ok.rs"
export EXAMPLE_SCRIPT_DIR="examples/"
cat << EoF > ./$EXAMPLE_SCRIPT_DIR/$EXAMPLE_SCRIPT_FILE
//FROM HERE
// https://doc.rust-lang.org/beta/src/core/result.rs.html
pub fn main(){

     let x: Result<i32, &str> = Ok(-3);
     assert_eq!(x.is_ok(), true);
    
     let x: Result<i32, &str> = Err("Some error message");
     assert_eq!(x.is_ok(), false);
}

/*
export FILE_NAME=$EXAMPLE_SCRIPT_FILE
export FILE_DIR_NAME=$EXAMPLE_SCRIPT_DIR
echo "build prg => \$(echo \$FILE_NAME | cut -d . -f 1)";
cargo build --example "\$(echo \$FILE_NAME | cut -d . -f 1)"
echo "run PRG => \$(echo \$FILE_NAME | cut -d . -f 1)";
cargo run --example "\$(echo \$FILE_NAME | cut -d . -f 1)"
echo "";
echo "run TEST => \$(echo \$FILE_NAME | cut -d . -f 1)"
cargo test --example "\$(echo \$FILE_NAME | cut -d . -f 1)"
# cargo test --jobs 4 --example "\$(echo \$FILE_NAME | cut -d . -f 1)"
echo "ReturnCode => \$?"
*/
EoF

```

### XX_testcase_fn_result_error_handling_main_fn.rs

````rust,no_run
#!/usr/bin/env bash
export EXAMPLE_SCRIPT_FILE="XX_divide_to_two_main_fn.rs"
export EXAMPLE_SCRIPT_DIR="examples/"
cat << EoF > ./$EXAMPLE_SCRIPT_DIR/$EXAMPLE_SCRIPT_FILE
// FORM HERE
// https://stackoverflow.com/questions/57234140/how-to-assert-io-errors-in-rust
// if let Ok ; if let Err(err);
// https://rust-classes.com/chapter_3_3
// next example
// https://stackoverflow.com/questions/62375381/an-elegant-way-of-getting-resultt-e-based-on-result-e1-and-resultt-e2
// 
// https://stackoverflow.com/questions/67422765/how-can-i-generate-an-errorerror-instance 


fn divide_to_two(n:i32) ->Result<i32, Box<dyn std::error::Error>> {
let result = n/2;
Ok(result)
}

fn main()-> Result<(), Box<dyn std::error::Error>> {

println!("{:?}",divide_to_two(4));

Ok(())
}//end of main

#[test]
fn test_success(){


}

/*
export FILE_NAME=$EXAMPLE_SCRIPT_FILE
export FILE_DIR_NAME=$EXAMPLE_SCRIPT_DIR
echo "build prg => \$(echo \$FILE_NAME | cut -d . -f 1)";
cargo build --example "\$(echo \$FILE_NAME | cut -d . -f 1)"
echo "run PRG => \$(echo \$FILE_NAME | cut -d . -f 1)";
cargo run --example "\$(echo \$FILE_NAME | cut -d . -f 1)"
echo "";
echo "run TEST => \$(echo \$FILE_NAME | cut -d . -f 1)"
cargo test --example "\$(echo \$FILE_NAME | cut -d . -f 1)"
# cargo test --jobs 4 --example "\$(echo \$FILE_NAME | cut -d . -f 1)"
echo "ReturnCode => \$?"
*/
EoF
```

## grep -r --include "*.sh" set "."

## [next step](https://github.com/assert-rs/assert_cmd/blob/master/examples/example_fixture.rs)

## LinuxExitCode

```rust,no_run
#!/usr/bin/env bash
export EXAMPLE_SCRIPT_FILE="98_linux_exit_code.rs"
export EXAMPLE_SCRIPT_DIR="examples/"
cat << EoF > ./$EXAMPLE_SCRIPT_DIR/$EXAMPLE_SCRIPT_FILE
// FROM HERE
// https://stackoverflow.com/questions/24245276/why-does-rust-not-have-a-return-value-in-the-main-function-and-how-to-return-a
use std::process::{ExitCode, Termination};

pub enum LinuxExitCode { ExitOK, ExitERR(u8) }

impl Termination for LinuxExitCode {
   fn report(self) -> ExitCode {
     match self {
       LinuxExitCode::ExitOK => ExitCode::SUCCESS,
       LinuxExitCode::ExitERR(v) => ExitCode::from(v)
     }
   }
}
fn main() -> LinuxExitCode {
    LinuxExitCode::ExitERR(3)
}


/*
export FILE_NAME=$EXAMPLE_SCRIPT_FILE
export FILE_DIR_NAME=$EXAMPLE_SCRIPT_DIR
echo "build prg => \$(echo \$FILE_NAME | cut -d . -f 1)";
cargo build --example "\$(echo \$FILE_NAME | cut -d . -f 1)"
echo "run PRG => \$(echo \$FILE_NAME | cut -d . -f 1)";
cargo run --example "\$(echo \$FILE_NAME | cut -d . -f 1)"
echo "";
echo "run TEST => \$(echo \$FILE_NAME | cut -d . -f 1)"
cargo test --example "\$(echo \$FILE_NAME | cut -d . -f 1)"
cargo test --jobs 4 --example "\$(echo \$FILE_NAME | cut -d . -f 1)"
echo "ReturnCode => $?"
*/
EoF

````

[Markdown marker from here](https://github.com/MathiasStadler/repo_template/blob/main/includes/markdown_marker.md#to-highlight-a-note-and-warning-using-blockquote)
