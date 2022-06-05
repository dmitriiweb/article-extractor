use scraper::{Html, Selector};
pub struct Title {
    pub title: String,
}

impl Title {
    pub fn from_html(html: Html) -> Self {
        let emptry_string = String::from("");
        let title_from_title = get_title_from_title(html);
        if title_from_title == emptry_string {
            return Title {
                title: emptry_string,
            };
        }
        return Title {
            title: title_from_title,
        };
    }
}

fn get_title_from_title(html: Html) -> String {
    let selector = Selector::parse("title").unwrap();
    let title = html.select(&selector).next();
    if title == None {
        return String::from("");
    }
    let title_text = title.unwrap().text().collect::<String>();
    title_text
}
