use super::titles_extractor::TitlesExtractor;
use regex::Regex;
use scraper::{Html, Selector};

#[derive(Debug)]
struct OptimalTitle {
    title_text: Option<String>,
    use_delimiter: bool,
}

pub fn get_title(html: &Html) -> Option<String> {
    let title_extractor = TitlesExtractor { html: html };
    let title_from_title = title_extractor.get_title();
    let title_from_h1 = title_extractor.get_h1();
    let title_from_fb = title_extractor.get_fb();
    println!("{:?}", title_from_h1);
    // let title_from_title = get_title_from_title(html);
    // if title_from_title == None {
    //     return None;
    // }
    // let title_from_h1 = get_title_from_h1(html);
    // let title_from_fb = get_title_from_fb(html);

    let title = extract_title(title_from_title, title_from_h1, title_from_fb);

    title
}

fn extract_title(
    title_title: Option<String>,
    title_h1: Option<String>,
    title_fb: Option<String>,
) -> Option<String> {
    let filter_re = Regex::new("[^\\u4e00-\\u9fa5a-zA-Z0-9]\x20]").unwrap();
    let optimal_title = get_optimal_title(title_title, title_h1, title_fb);
    // let title = split_title(&optimal_title);
    None
}

// fn split_title(title: &OptimalTitle) -> Option<String> {}

// fn title_spliter(title: &str, splitter: &str) -> String {}

fn get_optimal_title(
    title_title: Option<String>,
    title_h1: Option<String>,
    title_fb: Option<String>,
) -> OptimalTitle {
    let filtered_title = clean_title(&title_title);
    let filtered_h1 = clean_title(&title_h1);
    let filtered_fb = clean_title(&title_fb);
    let mut use_delimiter = false;
    let mut title_text = title_title.clone();

    if title_title.unwrap() == title_h1.clone().unwrap() {
        use_delimiter = true;
    } else if !filtered_h1.is_none() && filtered_h1 == filtered_fb {
        title_text = title_h1;
        use_delimiter = true;
    } else if !filtered_h1.is_none()
        && filtered_title
            .clone()
            .unwrap()
            .contains(&title_h1.clone().unwrap())
        && !filtered_fb.is_none()
        && filtered_title
            .clone()
            .unwrap()
            .contains(&filtered_fb.clone().unwrap())
        && filtered_h1.unwrap().len() > filtered_fb.clone().unwrap().len()
    {
        title_text = title_h1;
        use_delimiter = true;
    } else if !filtered_fb.is_none()
        && filtered_fb != filtered_title
        && filtered_title
            .unwrap()
            .starts_with(&title_fb.clone().unwrap())
    {
        title_text = title_fb;
        use_delimiter = true;
    }

    OptimalTitle {
        title_text,
        use_delimiter,
    }
}

fn clean_title(title: &Option<String>) -> Option<String> {
    if title.is_none() {
        return None;
    }
    let filter_re = Regex::new("[^\\u4e00-\\u9fa5a-zA-Z0-9]\x20]").unwrap();
    let filtered_title = filter_re
        .replace_all(&title.clone().unwrap(), "")
        .to_ascii_lowercase();

    Some(filtered_title)
}
