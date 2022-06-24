use std::process::Command;

/// From a computation given as input, returns the error and the output of
/// qalc in RPN mode.
pub fn rpn_qalc(computation: &str) -> String {
    let cmd = Command::new("qalc")
        .arg("-set")
        .arg("rpn on")
        .arg(computation)
        .output()
        .expect("Unable to start the `qalc` command");
    let err = std::str::from_utf8(&cmd.stderr).unwrap().to_string();
    let out = std::str::from_utf8(&cmd.stdout).unwrap().to_string();
    err + &out
}

