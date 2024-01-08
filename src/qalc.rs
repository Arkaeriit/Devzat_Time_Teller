/// From a computation given as input, returns the error and the output of
/// qalc in RPN mode.
pub fn rpn_calc(computation: &str) -> String {
    let replaced = computation.replace(":carrot:", "🥕");
    let replaced = replaced.replace("🥕", "^");
    match math_parse::MathParse::parse_rpn(&replaced) {
        Ok(parsed) => match parsed.solve_auto(None) {
            Ok(Ok(i))  => format!("{i}"),
            Ok(Err(f)) => format!("{f}"),
            Err(err)   => format!("{err}"),
        }
        Err(err) => format!("{err}")
    }
}

/// From a computation given as input, returns the error and the output of
/// qalc in infix mode.
pub fn infix_calc(computation: &str) -> String {
    let replaced = computation.replace(":carrot:", "🥕");
    let replaced = replaced.replace("🥕", "^");
    match math_parse::MathParse::parse(&replaced) {
        Ok(parsed) => match parsed.solve_auto(None) {
            Ok(Ok(i))  => format!("{i}"),
            Ok(Err(f)) => format!("{f}"),
            Err(err)   => format!("{err}"),
        }
        Err(err) => format!("{err}")
    }
}

