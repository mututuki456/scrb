use anyhow::Result;
use crate::scrapbox::api::ScrapboxClient;

pub fn run(project: &str, keyword: Option<&str>, sid: Option<&str>) -> Result<()> {
    let client = ScrapboxClient::new(project, sid);
    let pages = client.fetch_titles(false)?;

    for (i, page) in pages.iter().enumerate() {
        if keyword.map_or(true, |kw| page.title.contains(kw)) {
            println!("{:3}: {}", i + 1, page.title);
        }
    }

    Ok(())
}
