use assert_cmd::prelude::*;
use predicates::str::contains;
use std::process::Command;

#[test]
fn cli_no_args() {
    Command::cargo_bin("{{crate_name}}")
        .unwrap()
        .assert()
        .stdout(contains("Hello, world!"));
}
