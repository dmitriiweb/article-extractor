use crate::title;
use scraper::Html;

pub struct Article {
    pub title: Option<String>,
    pub text: String,
    pub article_html: String,
}

impl Article {
    pub fn from_html(html: String) -> Self {
        let html = Html::parse_document(&html[..]);
        let title = title::get_title(&html);
        Article {
            title: title,
            text: String::from("html"),
            article_html: String::from("html"),
        }
    }
}
