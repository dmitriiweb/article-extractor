use scraper::{Html, Selector};

/// Extracting article title from html
#[derive(Copy, Clone)]
pub struct TitlesExtractor<'a> {
    pub html: &'a Html,
}

impl<'a> TitlesExtractor<'a> {
    /// Extract article title from <title> tag
    pub fn get_title(self) -> Option<String> {
        let selector = Selector::parse("title").unwrap();
        let title = self.html.select(&selector).next();
        if title == None {
            return None;
        }
        let title_text = title.unwrap().text().collect::<String>();
        Some(title_text)
    }

    pub fn get_h1(self) -> Option<String> {
        /// Extract article title from <h1> tags
        let selector = Selector::parse("h1").unwrap();
        let h1_list = self.html.select(&selector);
        let mut titles: Vec<String> = Vec::new();
        for elem in h1_list {
            let text = elem.text().collect::<String>();
            titles.push(text);
        }
        if titles.len() == 0 {
            return None;
        }
        titles.sort_by(|b, a| a.len().cmp(&b.len()));
        let title_text = &titles[0];
        let title_text_list = title_text.split(" ").collect::<Vec<&str>>();
        if title_text_list.len() <= 2 {
            return None;
        }
        return Some(title_text.to_string());
    }

    /// Extract article title from Facebook meta tag
    pub fn get_fb(self) -> Option<String> {
        let selector1 = Selector::parse("meta[property=\"og:title\"]").unwrap();

        let mut title = self.html.select(&selector1).next();
        if title == None {
            let selector2 = Selector::parse("meta[name=\"og:title\"]").unwrap();
            title = self.html.select(&selector2).next();
        }
        if title == None {
            return None;
        }
        let title_text = title.unwrap().value().attr("content").unwrap();
        Some(title_text.to_string())
    }
}
