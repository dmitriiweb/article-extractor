use scraper::{Html, Selector};
pub struct Title {
    pub title: String,
}

impl Title {
    pub fn from_html(html: &Html) -> Self {
        let emptry_string = String::from("");
        let title_from_title = get_title_from_title(html);
        if title_from_title == emptry_string {
            return Title {
                title: emptry_string,
            };
        }

        let title_from_h1 = get_title_from_h1(html);
        return Title {
            title: title_from_h1,
        };
    }
}

fn get_title_from_h1(html: &Html) -> String {
    let selector = Selector::parse("h1").unwrap();
    let h1_list = html.select(&selector);
    let mut titles: Vec<String> = Vec::new();
    for elem in h1_list {
        let text = elem.text().collect::<String>();
        titles.push(text);
    }
    if titles.len() == 0 {
        return String::from("");
    }
    titles.sort_by(|a, b| a.len().cmp(&b.len()));
    titles.reverse();
    let title_text = &titles[0];
    let title_text_list = title_text.split(" ").collect::<Vec<&str>>();
    if title_text_list.len() <= 2 {
        return "".to_string();
    }
    return title_text.to_string();
}

fn get_title_from_title(html: &Html) -> String {
    let selector = Selector::parse("title").unwrap();
    let title = html.select(&selector).next();
    if title == None {
        return String::from("");
    }
    let title_text = title.unwrap().text().collect::<String>();
    title_text
}
