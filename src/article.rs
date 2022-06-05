pub struct Article {
    pub title: String,
    pub text: String,
    pub article_html: String,
}

impl Article {
    pub fn from_html(html: String) -> Self {
        Article {
            title: String::from("test title"),
            text: String::from("html"),
            article_html: String::from(html),
        }
    }
}
