# Explore RustTestcase MainFn Result

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

## simplest testcase for ```rust fn main()```

- we would to use [assert_cmd](https://crates.io/crates/assert_cmd)
-- modified this example - [first testcase](https://github.com/assert-rs/assert_cmd/blob/master/tests/cargo.rs)

### add assert_cmd crate to project

```bash
cargo add assert_cmd
```

### first simplest testcase - hello world

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
    let mut cmd = process::Command::cargo_bin("$(echo $SCRIPT_FILE | cut -d . -f 1)").unwrap();
    cmd.assert().success().stdout("Hello, world!\n");
}

/*
export FILE_NAME=$EXAMPLE_SCRIPT_FILE
export FILE_DIR_NAME=$EXAMPLE_SCRIPT_DIR
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
cargo run --example "\$(echo \$FILE_NAME | cut -d . -f 1)"
# rust test
cargo test --example "\$(echo \$FILE_NAME | cut -d . -f 1)"
echo "ReturnCode => \$?"
*/

EoF


```

## [next step](https://github.com/assert-rs/assert_cmd/blob/master/examples/example_fixture.rs)

[marker from here](https://github.com/MathiasStadler/repo_template/blob/main/includes/markdown_marker.md#to-highlight-a-note-and-warning-using-blockquote)
