use crate::scrapbox::model::ScrapboxPage;
use crate::markdown::link::*;
use crate::markdown::list::*;
use crate::markdown::style::*;
use crate::markdown::block::*;

pub fn normalize_line(line: &str) -> String {
    if let Some(styled) = apply_style(line) {
        return styled;
    }

    let line = image_link(line);
    let line = external_link(&line);
    let line = internal_link(&line);

    line
}

pub fn convert_line(line: &str) -> String {
    if let Some(list) = convert_list(line) {
        return list;
    }

    normalize_line(line)
}

pub fn to_markdown(page: &ScrapboxPage) -> String {
    let mut out = format!("# {}\n\n", &page.title);

    let mut i = 0;
    while i < page.lines.len(){
        let line = &page.lines[i];

        if let Some(block) = try_code_block(&page.lines, i) {
            out.push_str(&block.output);
            i = block.next_index;
            continue;
        }
 
        if let Some(block) = try_table_block(&page.lines, i) {
            out.push_str(&block.output);
            i = block.next_index;
            continue;
        }
 
        out.push_str(&convert_line(&line.text));
        out.push('\n');
        i += 1;
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scrapbox::model::ScrapboxLine;

    #[test]
    fn heading_level1() {
        assert_eq!(convert_line("[* タイトル]"), "**タイトル**");
    }

    #[test]
    fn heading_level2() {
        assert_eq!(convert_line("[** タイトル]"), "### タイトル");
    }

    #[test]
    fn heading_level3() {
        assert_eq!(convert_line("[*** タイトル]"), "## タイトル");
    }

    #[test]
    fn decoration_alert() {
        assert_eq!(convert_line("[! 注意事項]"), "注意事項");
    }

    #[test]
    fn decoration_green() {
        assert_eq!(convert_line("[# 緑文字]"), "緑文字");
    }

    #[test]
    fn decoration_quote() {
        assert_eq!(convert_line("[> 引用文]"), "引用文");
    }

    #[test]
    fn decoration_multi() {
        assert_eq!(convert_line("[!#% 複合装飾]"), "複合装飾");
    }

    #[test]
    fn list_level_1() {
        assert_eq!(convert_line(" test"), "- test");
    }

    #[test]
    fn list_level_2() {
        assert_eq!(convert_line("  test"), "   - test");
    }

    #[test]
    fn external_link() {
        assert_eq!(
            convert_line("[サイト https://example.com]"),
            "[サイト](https://example.com)"
        );
    }

    #[test]
    fn internal_link() {
        assert_eq!(
            convert_line("これは [用語解説] の説明"),
            "これは 用語解説 の説明"
        );
    }

    #[test]
    fn normal_text() {
        assert_eq!(convert_line("Hello World"), "Hello World");
    }

    #[test]
    fn code_block() {
        let page = ScrapboxPage {
            title: "test".to_string(),
            lines: vec![
                ScrapboxLine {text: "code:test.rs".to_string()},
                ScrapboxLine {text: " fn main() {".to_string()},
                ScrapboxLine {text: "     println!(\"hello\");".to_string()},
                ScrapboxLine {text: " }".to_string()},
            ],
        };

        let md = to_markdown(&page);

        assert!(md.contains("```"));
    }

    #[test]
    fn table_block() {
        let page = ScrapboxPage {
            title: "test".to_string(),
            lines: vec![
                ScrapboxLine {text: "table:test".to_string()},
                ScrapboxLine {text: " title\tcontent".to_string()},
                ScrapboxLine {text: " test_01\texample_text_01".to_string()},
                ScrapboxLine {text: " test_02\texample_text_02".to_string()},
            ],
        };

        let md = to_markdown(&page);

        assert!(md.contains("| title | content |"));
        assert!(md.contains("| --- | --- |"));
        assert!(md.contains("| test_01 | example_text_01 |"));
        assert!(md.contains("| test_02 | example_text_02 |"));
    }
}
