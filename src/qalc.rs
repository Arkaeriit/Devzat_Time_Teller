use std::process::Command;

/// From a computation given as input, returns the error and the output of
/// qalc in RPN mode.
pub fn rpn_qalc(computation: &str) -> String {
    match Command::new("qalc")
        .arg("-set")
        .arg("rpn on")
        .arg(computation)
        .output() {
            Ok(cmd) => std::str::from_utf8(&cmd.stderr).unwrap().to_string() +
                std::str::from_utf8(&cmd.stdout).unwrap(),
            Err(_) => "Error, qalc not found on the host system.".to_string(),
    }
}

