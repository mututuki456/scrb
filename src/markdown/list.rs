use crate::markdown::convert::normalize_line;

pub fn convert_list(line: &str) -> Option<String> {
    let indent = line.chars().take_while(|c| *c == ' ').count();

    if indent == 0 {
        return None;
    }

    let text = line.trim_start();

    Some(format!(
        "{}- {}",
        "   ".repeat(indent - 1),
        normalize_line(text)
    ))
}

