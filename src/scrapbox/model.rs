use serde::Serialize;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ScrapboxPage {
    pub title: String,
    pub lines: Vec<ScrapboxLine>,
}

#[derive(Debug, Deserialize)]
pub struct ScrapboxLine {
    pub text: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SearchTitle {
    pub id: String,
    pub title: String,

    #[serde(default)]
    pub links: Vec<String>,

    pub updated: i64,
}
