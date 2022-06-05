use article_extractor::article::Article;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
struct TestData {
    title: String,
    text: String,
}

impl TestData {
    fn from_file(file_path: PathBuf) -> Option<Self> {
        let json_str = read_file(file_path);
        let td: TestData = serde_json::from_str(&json_str[..]).unwrap();
        return Some(td);
    }
}

fn get_data_dir() -> PathBuf {
    let mut base_dir = env::current_dir().expect("Error");
    base_dir.push("tests");
    base_dir.push("data");
    return base_dir;
}

fn read_file(file_path: PathBuf) -> String {
    fs::read_to_string(file_path).expect("Cannot read")
}

#[test]
fn test_simple_article() {
    let data_dir = get_data_dir();
    let mut html_file = data_dir.clone();
    html_file.push("example.html");
    let mut test_data_file = data_dir.clone();
    test_data_file.push("example.json");

    let html_content = read_file(html_file);
    let new_article = Article::from_html(html_content);
    let test_data = TestData::from_file(test_data_file).unwrap();

    assert_eq!(new_article.title, test_data.title);
}

#[test]
fn test_article_without_title() {
    let data_dir = get_data_dir();
    let mut html_file = data_dir.clone();
    html_file.push("example_without_title.html");
    let mut test_data_file = data_dir.clone();
    test_data_file.push("example_without_title.json");

    let html_content = read_file(html_file);
    let new_article = Article::from_html(html_content);
    let test_data = TestData::from_file(test_data_file).unwrap();

    assert_eq!(new_article.title, test_data.title);
}
