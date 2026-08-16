use std::io::Write;
use std::process::{Command, Output, Stdio};

fn run_cli(input: &str) -> Output {
    let mut child = Command::new(env!("CARGO_BIN_EXE_ccml-cli"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn ccml-cli");

    child
        .stdin
        .take()
        .expect("open ccml-cli stdin")
        .write_all(input.as_bytes())
        .expect("write ccml-cli stdin");

    child.wait_with_output().expect("wait for ccml-cli")
}

#[test]
fn successful_transcode_writes_only_json_to_stdout() {
    let output = run_cli("a: 1");

    assert_eq!(output.status.code(), Some(0));
    assert_eq!(String::from_utf8(output.stdout).unwrap(), "{\n  \"a\": 1\n}\n");
    assert!(output.stderr.is_empty());
}

#[test]
fn duplicate_key_writes_warning_to_stderr_and_succeeds() {
    let output = run_cli("x: 1\nx: 2");

    assert_eq!(output.status.code(), Some(0));
    assert_eq!(String::from_utf8(output.stdout).unwrap(), "{\n  \"x\": 2\n}\n");
    assert_eq!(
        String::from_utf8(output.stderr).unwrap(),
        "2:1 warning CCML2001 duplicate key 'x' overwritten by keep-last policy\n"
    );
}

#[test]
fn parse_error_preserves_stderr_and_failure_exit_code() {
    let output = run_cli("a:");

    assert_eq!(output.status.code(), Some(1));
    assert!(output.stdout.is_empty());
    assert_eq!(
        String::from_utf8(output.stderr).unwrap(),
        "1:3 error CCML1001 missing value\n"
    );
}
