use serde::Deserialize;

// Interfaces.

#[derive(Deserialize)]
pub enum Status {
    Success,
    Error,
    Unknown,
}

#[derive(Debug, Deserialize, Default, Clone, PartialEq, Eq)]
pub struct IArticle {
    pub id: i32,
    pub title: String,
    pub pub_date: String,
    pub published: bool,
    pub headline: String,
    pub image: String,
    pub content: String,
}