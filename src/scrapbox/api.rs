use anyhow::Result;
use urlencoding::encode;
use reqwest::blocking::Client;
use dialoguer::Select;
use std::time::{Duration, SystemTime};
use std::path::PathBuf;

use super::model::{ScrapboxPage, SearchTitle};

const CACHE_TTL: Duration = Duration::from_secs(86400);

pub struct ScrapboxClient {
    client: Client,
    project: String,
    sid: Option<String>,
}

impl ScrapboxClient {
    pub fn new(project: &str, sid: Option<&str>) -> Self {
        Self {
            client: Client::new(),
            project: project.to_string(),
            sid: sid.map(|s| s.to_string()),
        }
    }

    fn with_auth(&self, request: reqwest::blocking::RequestBuilder) -> reqwest::blocking::RequestBuilder {
        match &self.sid {
            Some(sid) => request.header(reqwest::header::COOKIE, format!("connect.sid={sid}")),
            None => request,
        }
    }

    pub fn fetch_page(&self, page: &str) -> Result<ScrapboxPage> {
        let page = encode(page);
        let url = format!("https://scrapbox.io/api/pages/{}/{page}", self.project);

        let response = self
            .with_auth(self.client.get(&url))
            .send()?
            .error_for_status()?
            .json()?;

        Ok(response)
    }

    pub fn fetch_titles(&self, refresh: bool) -> Result<Vec<SearchTitle>> {
        let cache_path = self.cache_path();
 
        if !refresh {
            if let Some(titles) = self.load_cache(&cache_path) {
                return Ok(titles);
            }
        }
 
        let titles = self.fetch_titles_from_api()?;
        self.save_cache(&cache_path, &titles)?;
 
        Ok(titles)
    }

    pub fn fetch_titles_from_api(&self) -> Result<Vec<SearchTitle>> {
        let url = format!("https://scrapbox.io/api/pages/{}/search/titles", self.project);

        let response = self
            .with_auth(self.client.get(url))
            .send()?
            .error_for_status()?
            .json()?;

        Ok(response)
    }

    pub fn resolve_page(&self, query: &str) -> Result<String> {
        let pages = self.fetch_titles(false)?;
        let matches = Self::filter_titles(pages, query);

        match matches.len() {
            0 => anyhow::bail!("ページが見つかりません"),
            1 => Ok(matches[0].title.clone()),
            _ => Self::select_page(matches),
        }
    }

 
    fn cache_path(&self) -> PathBuf {
        let exe_dir = std::env::current_exe()
            .ok()
            .and_then(|p| p.parent().map(|p| p.to_path_buf()))
            .unwrap_or_else(|| std::path::PathBuf::from("."));
 
        exe_dir.join("cache").join(&self.project).join("titles.json")
    }
 
    fn load_cache(&self, path: &std::path::Path) -> Option<Vec<SearchTitle>> {
        let meta = std::fs::metadata(path).ok()?;
        let modified = meta.modified().ok()?;
        let age = SystemTime::now().duration_since(modified).ok()?;
 
        if age > CACHE_TTL {
            return None;
        }
 
        let content = std::fs::read_to_string(path).ok()?;
        serde_json::from_str(&content).ok()
    }
 
    fn save_cache(&self, path: &std::path::Path, titles: &[SearchTitle]) -> Result<()> {
        if let Some(dir) = path.parent() {
            std::fs::create_dir_all(dir)?;
        }
        let content = serde_json::to_string(titles)?;
        std::fs::write(path, content)?;
        Ok(())
    }
 
    fn filter_titles(pages: Vec<SearchTitle>, query: &str) -> Vec<SearchTitle> {
        let query = query.to_lowercase();
        pages
            .into_iter()
            .filter(|p| p.title.to_lowercase().contains(&query))
            .collect()
    }

    fn select_page(matches: Vec<SearchTitle>) -> Result<String> {
        let items: Vec<String> = matches.into_iter().map(|p| p.title).collect();

        let selection = Select::new()
            .with_prompt("?select get ScrapboxPage (Esc: cancel)")
            .items(&items)
            .default(0)
            .interact_opt()?;

        match selection {
            Some(i) => Ok(items[i].clone()),
            None => anyhow::bail!("select cancel..."),
        }
    }
}
