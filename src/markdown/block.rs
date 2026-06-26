use crate::scrapbox::model::ScrapboxLine;

pub struct ConsumedBlock {
    pub output: String,
    pub next_index: usize,
}

pub fn try_code_block(lines: &[ScrapboxLine], i: usize) -> Option<ConsumedBlock> {
    if !lines[i].text.starts_with("code:") {
        return None;
    }

    let mut out = String::from("```\n");
    let mut j = i + 1;

    while j < lines.len() && lines[j].text.starts_with(' ') {
        out.push_str(lines[j].text.strip_prefix(' ').unwrap_or(&lines[j].text));
        out.push('\n');
        j += 1;
    }

    out.push_str("```\n");

    Some(ConsumedBlock { output: out, next_index: j })
}

pub fn try_table_block(lines: &[ScrapboxLine], i: usize) -> Option<ConsumedBlock> {
    if !lines[i].text.starts_with("table:") {
        return None;
    }

    let mut rows: Vec<Vec<String>> = Vec::new();
    let mut j = i + 1;

    while j < lines.len() {
        let row = &lines[j].text;
        if !row.starts_with(' ') {
            break;
        }
        rows.push(
            row.trim_start()
                .split('\t')
                .map(|s| s.trim().to_string())
                .collect(),
        );
        j += 1;
    }

    if rows.is_empty() {
        return Some(ConsumedBlock { output: String::new(), next_index: j });
    }

    let mut out = String::new();

    out.push('|');
    for cell in &rows[0] {
        out.push_str(&format!(" {} |", cell));
    }
    out.push('\n');

    out.push('|');
    for _ in &rows[0] {
        out.push_str(" --- |");
    }
    out.push('\n');

    for row in rows.iter().skip(1) {
        out.push('|');
        for cell in row {
            out.push_str(&format!(" {} |", cell));
        }
        out.push('\n');
    }

    out.push('\n');

    Some(ConsumedBlock { output: out, next_index: j })
}
