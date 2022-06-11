use super::titles_extractor::TitlesExtractor;
use regex::Regex;
use scraper::{Html, Selector};

#[derive(Debug, Clone)]
struct OptimalTitle {
    title_text: Option<String>,
    used_delimiter: bool,
    title_h1_text: Option<String>,
}

/// Extracts the title from the HTML document.
pub fn get_title(html: &Html) -> Option<String> {
    let title_extractor = TitlesExtractor { html };
    let title_from_title = title_extractor.get_title();
    if title_from_title == None {
        return None;
    }
    let title_from_h1 = title_extractor.get_h1();
    let title_from_fb = title_extractor.get_fb();

    let title = get_optimal_and_clean_title(title_from_title, title_from_h1, title_from_fb);

    title
}

fn get_optimal_and_clean_title(
    title_title: Option<String>,
    title_h1: Option<String>,
    title_fb: Option<String>,
) -> Option<String> {
    let optimal_title = get_optimal_title(title_title, title_h1, title_fb);
    let title = clean_optimal_title(&optimal_title);
    title
}

/// get form the title the optimal part
fn clean_optimal_title(opt_title: &OptimalTitle) -> Option<String> {
    if opt_title.title_text.is_none() || opt_title.used_delimiter {
        return opt_title.title_text.clone();
    }
    let chars_list = ["|", " - ", "_", "/", " » "];
    let mut title = opt_title.title_text.clone().unwrap();
    for i in chars_list.iter() {
        title = title_splitter(&title, i, &opt_title.title_h1_text);
    }
    Some(title)
}

fn title_splitter(title: &str, splitter: &str, hint: &Option<String>) -> String {
    let mut large_text_len = 0;
    let mut large_text_idx = 0;
    let splitted_titles = title.split(splitter).collect::<Vec<&str>>();
    let mut hint_text = "".to_string();

    let filter_re = Regex::new("[^a-zA-Z0-9\x20]").unwrap();
    if !hint.is_none() {
        hint_text = filter_re
            .replace_all(&hint.clone().unwrap(), "")
            .to_ascii_lowercase();
    }

    for (idx, splitted_title) in splitted_titles.iter().enumerate() {
        let current_text = splitted_title.trim();
        if !hint.is_none() && hint_text != "" {
            let filtered_current_text = filter_re
                .replace_all(&current_text, "")
                .to_ascii_lowercase();
            if filtered_current_text.contains(&filtered_current_text) {
                large_text_idx = 1;
                break;
            }
            if current_text.len() > large_text_len {
                large_text_len = current_text.len();
                large_text_idx = idx;
            }
        }
    }

    let title = splitted_titles[large_text_idx].trim().to_string();
    title
}

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

    if title_h1.is_none() {
        return OptimalTitle {
            title_text,
            used_delimiter: false,
            title_h1_text: None,
        };
    }

    if title_title.unwrap() == title_h1.clone().unwrap() {
        use_delimiter = true;
    } else if !filtered_h1.is_none() && filtered_h1 == filtered_fb {
        title_text = title_h1.clone();
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
        title_text = title_h1.clone();
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
        used_delimiter: use_delimiter,
        title_h1_text: title_h1,
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
