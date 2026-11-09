use pretty_assertions::assert_eq;
use std::process::Command;

#[test]
fn prints_the_result_and_exits_successfully() {
    let output = Command::new(env!("CARGO_BIN_EXE_{{project-name}}"))
        .output()
        .expect("failed to launch the executable");

    assert!(output.status.success(), "{output:?}");
    assert_eq!(String::from_utf8(output.stdout).unwrap(), "2 + 2 = 4\n");
    assert!(output.stderr.is_empty());
}
