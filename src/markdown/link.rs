use regex::Regex;

pub fn image_link(line: &str) -> String {
    let re = Regex::new(
        r"^\[(https?://.+\.(png|jpg|jpeg|gif|webp))\]$"
    ).unwrap();

    re.replace_all(line, "![]($1)").to_string()
}

pub fn external_link(line: &str) -> String {
    let re = Regex::new(
        r"\[([^\]]+?) (https?://[^\]]+)\]"
    ).unwrap();

    re.replace_all(line, "[$1]($2)").to_string()
}

pub fn internal_link(line: &str) -> String {
    let re = Regex::new(r"\[([^\[\]]+)\]").unwrap();

    let mut out = String::new();
    let mut last = 0;

    for cap in re.captures_iter(line) {
        let m = cap.get(0).unwrap();

        out.push_str(&line[last..m.start()]);

        if line[m.end()..].starts_with('(') {
            out.push_str(m.as_str());
        } else {
            out.push_str(&cap[1]);
        }

        last = m.end();
    }

    out.push_str(&line[last..]);

    out
}
