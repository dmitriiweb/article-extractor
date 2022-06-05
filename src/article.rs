use crate::title::Title;
use scraper::Html;

pub struct Article {
    pub title: String,
    pub text: String,
    pub article_html: String,
}

impl Article {
    pub fn from_html(html: String) -> Self {
        let html = Html::parse_document(&html[..]);
        let title = Title::from_html(&html);
        Article {
            title: title.title,
            text: String::from("html"),
            article_html: String::from("html"),
        }
    }
}
