use regex::Regex;

#[derive(Default)]
pub struct Decoration {
    pub bold: bool,
    pub yellow_marker: bool,
    pub green_marker: bool,
    pub pink_marker: bool,
    pub red_strike: bool,
    pub quote: bool,
}

pub fn apply_style(line: &str) -> Option<String> {
    if let Some(head) = heading(line) {
        return Some(head);
    }

    if let Some((_deco, text)) = parse_decoration(line) {
        return Some(text);
    }

    None
}

pub fn heading(line: &str) -> Option<String> {
    if let Some(text) = line.strip_prefix("[*** ") {
        return Some(format!("## {}", text.trim_end_matches(']')));
    }

    if let Some(text) = line.strip_prefix("[** ") {
        return Some(format!("### {}", text.trim_end_matches(']')));
    }

    if let Some(text) = line.strip_prefix("[* ") {
        return Some(format!("**{}**", text.trim_end_matches(']')));
    }

    None
}

pub fn parse_decoration(line: &str) -> Option<(Decoration, String)> {
    let re = Regex::new(r#"^\[([!#%"*>]+)\s+(.+)\]$"#).unwrap();
    let caps = re.captures(line)?;

    let flags = caps.get(1)?.as_str();
    let text = caps.get(2)?.as_str();

    let mut deco = Decoration::default();

    for c in flags.chars() {
        match c {
            '*' => deco.bold = true,
            '"' => deco.yellow_marker = true,
            '#' => deco.green_marker = true,
            '%' => deco.pink_marker = true,
            '!' => deco.red_strike = true,
            '>' => deco.quote = true,
            _ => {}
        }
    }

    Some((deco, text.to_string()))
}
