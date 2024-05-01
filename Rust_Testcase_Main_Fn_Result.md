# Explore RustTestcase MainFn Result

## project setup for easy to used

### update rust of latest stable version

```bash
rustup update
```

### update all rust extension

#### install cargo-update

```bash
cargo install cargo-update
```

#### Self-update of cargo-update

- cargo-update will update itself seamlessly on Linux and Windows.

```bash
cargo-update
```

#### update all extension

```bash
cargo install-update -a
```

### check all necessary plugins installed

```bash
ls -la  ~/.cargo/bin/
```

### check is cargo-edit

#### is already installed

```bash
cargo install --list |grep cargo-edit
```

#### install cargo install /w command cargo add

```bash
cargo install cargo-edit
```

## rust switch from stable to nightly and back [found here](https://stackoverflow.com/questions/58226545/how-to-switch-between-rust-toolchains)

```bash
rustup override set nightly
```

back to stable

```bash
rustup override set stable
```



## simplest testcase for ```rust fn main()```

- we would to [use assert_cmd](https://crates.io/crates/assert_cmd)
-- [modified this example - first testcase](https://github.com/assert-rs/assert_cmd/blob/master/tests/cargo.rs)

-add assert_cmd crate to project

```bash
cargo add cargo add assert_cmd
```

```rust
#!/usr/bin/env bash
export EXAMPLE_SCRIPT_FILE="01_simplest_testcase.rs"
export EXAMPLE_SCRIPT_DIR="examples/"
cat << EoF > ./$EXAMPLE_SCRIPT_DIR/$EXAMPLE_SCRIPT_FILE
fn main() {
    println!("Hello, world!");
}

#[test]
fn cargo_binary() {
    let mut cmd = process::Command::cargo_bin("bin_fixture").unwrap();
    cmd.env("stdout", "42");
    cmd.assert().success().stdout("42\n");
}

EoF
```
