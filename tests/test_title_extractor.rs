use article_extractor::title::titles_extractor::TitlesExtractor;
use scraper::Html;

#[test]
fn test_title_tag() {
    let html = r#"
        <!DOCTYPE html>
        <head>
        <title>Title</title>
        </head>
        </html>
        <body></body>
    "#;

    let html_struct = Html::parse_document(html);
    let te = TitlesExtractor { html: &html_struct };
    let title = te.get_title();
    assert_eq!(title.unwrap(), "Title");
}

#[test]
fn test_title_no_title_tag() {
    let html = r#"
        <!DOCTYPE html>
        <head>
        </head>
        </html>
        <body></body>
    "#;

    let html_struct = Html::parse_document(html);
    let te = TitlesExtractor { html: &html_struct };
    let title = te.get_title();
    assert_eq!(title, None);
}

#[test]
fn test_title_from_h1() {
    let html = r#"
        <!DOCTYPE html>
        <head>
        </head>
        </html>
        <body><h1>Test title for h1</h1></body>
    "#;

    let html_struct = Html::parse_document(html);
    let te = TitlesExtractor { html: &html_struct };
    let title = te.get_h1();
    assert_eq!(title.unwrap(), "Test title for h1");
}
#[test]
fn test_title_from_h1_small() {
    let html = r#"
        <!DOCTYPE html>
        <head>
        </head>
        </html>
        <body><h1>Test</h1></body>
    "#;

    let html_struct = Html::parse_document(html);
    let te = TitlesExtractor { html: &html_struct };
    let title = te.get_h1();
    assert_eq!(title, None);
}

#[test]
fn test_title_from_fb_property() {
    let html = r#"
        <!DOCTYPE html>
        <head>
        <meta property="og:title" content="Test Title"/>
        </head>
        </html>
    "#;

    let html_struct = Html::parse_document(html);
    let te = TitlesExtractor { html: &html_struct };
    let title = te.get_fb();
    assert_eq!(title.unwrap(), "Test Title");
}

#[test]
fn test_title_from_fb_name() {
    let html = r#"
        <!DOCTYPE html>
        <head>
        <meta name="og:title" content="Test Title"/>
        </head>
        </html>
    "#;

    let html_struct = Html::parse_document(html);
    let te = TitlesExtractor { html: &html_struct };
    let title = te.get_fb();
    assert_eq!(title.unwrap(), "Test Title");
}
