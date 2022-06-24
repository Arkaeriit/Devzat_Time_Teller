use std::process::Command;

const TZ_PATH: &str = "/usr/share/zoneinfo/";

/// Return true if the timezone given as argument is valid and false otherwise.
fn is_tz_valid(tz: &str) -> bool {
    let tz_path = TZ_PATH.to_owned() + tz;
    let output = Command::new("find")
        .arg(tz_path)
        .output()
        .expect("find command failed to start");  
    output.status.success()
}

/// Returns the time at the timezone given in argument.
/// It is assumed that the given timezone is valid.
fn time_at_valid_tz(tz: &str) -> String {
    let mut output = Command::new("date")
        .arg("+%H:%M")
        .env("TZ", tz)
        .output()
        .expect("date command failed to start")
        .stdout;
    let last_char = output.len() - 1;
    output[last_char] = 0; // Remove trailing new line
    std::str::from_utf8(&output).unwrap().to_string() 
}

/// If the timezone given as argument is valid, returns the time there as a
/// string. Otherwise, returns None.
pub fn time_at_tz(tz: &str) -> Option<String> {
    match is_tz_valid(tz) {
        true  => Some(time_at_valid_tz(tz)),
        false => None,
    }
}


