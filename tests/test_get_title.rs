use article_extractor::title::title::get_title;
use scraper::Html;

#[test]
fn test_title_tag() {
    let html = r#"
        <!DOCTYPE html>
        <head>
        <title>Some test title</title>
        </head>
        </html>
        <body></body>
    "#;

    let html_struct = Html::parse_document(html);
    let title = get_title(&html_struct);
    assert_eq!(title.unwrap(), "Some test title");
}

#[test]
fn test_title_no_title_tag() {
    let html = r#"
        <!DOCTYPE html>
        <head>
        </head>
        </html>
        <body><h1>Some test title</h1></body>
    "#;

    let html_struct = Html::parse_document(html);
    let title = get_title(&html_struct);
    assert_eq!(title, None);
}

#[test]
fn test_title_from_h1() {
    let html = r#"
        <!DOCTYPE html>
        <head>
        <title>Some test title | website.name</title>
        </head>
        </html>
        <body><h1>Test title for h1</h1></body>
    "#;

    let html_struct = Html::parse_document(html);
    let title = get_title(&html_struct);
    assert_eq!(title.unwrap(), "Test title for h1");
}
#[test]
fn test_title_from_h1_small() {
    let html = r#"
        <!DOCTYPE html>
        <head>
        <title>Some test title | website.name</title>
        </head>
        </html>
        <body><h1>Some Test</h1></body>
    "#;

    let html_struct = Html::parse_document(html);
    let title = get_title(&html_struct);
    assert_eq!(title.unwrap(), "Some Test");
}
