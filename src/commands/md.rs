use anyhow::Result;
use crate::scrapbox::api::ScrapboxClient;
use crate::markdown::convert::to_markdown;

pub fn run(project: &str, page: &str, sid: Option<&str>) -> Result<()> {
    let client = ScrapboxClient::new(project, sid);

    let title = client.resolve_page(page)?;
    let page = client.fetch_page(&title)?;

    let md = to_markdown(&page);

    println!("{md}");

    Ok(())
}
