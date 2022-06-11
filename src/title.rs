use scraper::{Html, Selector};

pub fn from_html(html: &Html) -> Option<String> {
    let title_from_title = get_title_from_title(html);
    if title_from_title == None {
        return None;
    }

    let title_from_h1 = get_title_from_h1(html);
    let title_from_fb = get_title_from_fb(html);

    title_from_fb
}

fn get_title_from_fb(html: &Html) -> Option<String> {
    let selector1 = Selector::parse("");
    return None;
}

fn get_title_from_h1(html: &Html) -> Option<String> {
    let selector = Selector::parse("h1").unwrap();
    let h1_list = html.select(&selector);
    let mut titles: Vec<String> = Vec::new();
    for elem in h1_list {
        let text = elem.text().collect::<String>();
        titles.push(text);
    }
    if titles.len() == 0 {
        return None;
    }
    titles.sort_by(|b, a| a.len().cmp(&b.len()));
    println!("{:?}", titles);
    let title_text = &titles[0];
    let title_text_list = title_text.split(" ").collect::<Vec<&str>>();
    if title_text_list.len() <= 2 {
        return None;
    }
    return Some(title_text.to_string());
}

fn get_title_from_title(html: &Html) -> Option<String> {
    let selector = Selector::parse("title").unwrap();
    let title = html.select(&selector).next();
    if title == None {
        return None;
    }
    let title_text = title.unwrap().text().collect::<String>();
    Some(title_text)
}
