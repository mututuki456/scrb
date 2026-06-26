use anyhow::Result;
use crate::scrapbox::api::ScrapboxClient;

pub fn run(project: &str, page: &str, sid: Option<&str>) -> Result<()> {
    let client = ScrapboxClient::new(project, sid);

    let title = client.resolve_page(page)?;
    let content = client.fetch_page(&title)?;

    for line in content.lines {
        println!("{}", line.text);
    }

    Ok(())
}
