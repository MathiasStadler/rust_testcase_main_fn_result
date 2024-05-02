fn main() {
    println!("Hello, world!");
}

#[test]
fn cargo_binary() {
    let mut cmd = process::Command::cargo_bin("bin_fixture").unwrap();
    cmd.env("stdout", "42");
    cmd.assert().success().stdout("42\n");
}

/*
export FILE_NAME=01_simplest_testcase.rs
export FILE_DIR_NAME=examples/
git add $FILE_DIR_NAME/$FILE_NAME
git commit --all --message="-> Add BEFORE housekeeping => $FILE_DIR_NAME/$FILE_NAME"
# git push
# cargo install --list
# cargo update --workspace
cargo clippy --fix
cargo clippy --fix --examples
# cargo check --verbose
# cargo check --verbose --examples
cargo check
cargo check --examples
cargo fmt -- --emit=files $FILE_DIR_NAME/$FILE_NAME
git commit --all --message="-> Add AFTER housekeeping => $FILE_DIR_NAME/$FILE_NAME"
git push
cargo run --example "$(echo $FILE_NAME | cut -d . -f 1)"
echo "ReturnCode => $?"
*/
