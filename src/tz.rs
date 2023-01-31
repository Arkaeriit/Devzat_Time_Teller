use std::process::Command;

const TZ_PATH: &str = "/usr/share/zoneinfo/";

/// Try to fix timezones like PDT or PST into ones that work with the rest of
/// the system like PST8PDT
fn quick_patch_tz(tz: &str) -> &str {
    match tz {
        "PST" | "PDT" => "PST8PDT",
        "CST" | "CDT" => "CST6CDT",
        "EST" | "EDT" => "EST5EDT",
        "MT" => "America/Phoenix",
        _ => tz,
    }
}

/// Return true if the timezone given as argument is valid and false otherwise.
fn is_tz_valid(tz: &str) -> bool {
    if tz.contains(".") {
        return false;
    }
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
    let output = Command::new("date")
        .arg("+%H:%M")
        .env("TZ", tz)
        .output()
        .expect("date command failed to start")
        .stdout;
    std::str::from_utf8(&output).unwrap().trim().to_string()
}

/// If the timezone given as argument is valid, returns the time there as a
/// string. Otherwise, returns None.
pub fn time_at_tz(tz: &str) -> Option<String> {
    let filtered_tz = quick_patch_tz(tz);
    match is_tz_valid(filtered_tz) {
        true  => Some(time_at_valid_tz(filtered_tz)),
        false => None,
    }
}

