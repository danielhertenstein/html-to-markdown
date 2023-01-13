use reqwest::{Client, Result};
use scraper::{ElementRef, Html, Node::{Element, Text}, Selector};
use std::{fs::File, io::Write};
use std::path::PathBuf;
use url::Url;

#[tokio::main]
async fn main() -> Result<()> {
    let url = "https://www.sacdsa.org/blog/2020/07/06/a-people-of-color-s-history-of-dsa-part-4-DSA-Looks-Inward/";
    let client = Client::new();
    let raw_html = client.get(url)
        .send()
        .await?
        .text()
        .await?;

    let document = Html::parse_document(&raw_html);

    // TODO: Cannot use `?` because the error type is not the Reqwest error.
    let container_selector = Selector::parse(r#"div[class="container"]"#).unwrap();
    let content: Vec<ElementRef> = document.select(&container_selector).collect();
    // TODO: What is the proper way to check how many of these there are?
    assert_eq!(content.len(), 1);
    let content = content[0];

    let path = create_file_path(url);
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).unwrap();
    let mut file = match File::create(&path) {
        Err(why) => panic!("Couldn't create {}: {}", path.display(), why),
        Ok(file) => file,
    };

    // Start of liquid header
    writeln!(file, "---\nlayout: page").unwrap();

    let title_selector = Selector::parse("h1").unwrap();
    let title: Vec<ElementRef> = content.select(&title_selector).collect();
    assert_eq!(title.len(), 1);
    let title = title[0];
    writeln!(file, "title: \"{}\"", replace_html_entities(&title.inner_html()).trim()).unwrap();

    // End of liquid header
    writeln!(file, "permalink: /{}/\n---", path.with_extension("").display()).unwrap();

    // TODO: Where does the date go?
    let date_selector = Selector::parse(r#"p[class="text-light"]"#).unwrap();
    let date: Vec<ElementRef> = content.select(&date_selector).collect();
    assert_eq!(date.len(), 1);
    let date = date[0];
    println!("Date: {}", replace_html_entities(&date.inner_html()).trim());

    let body_selector = Selector::parse(r#"div[class="quill-output"]"#).unwrap();
    let body: Vec<ElementRef> = content.select(&body_selector).collect();
    assert_eq!(body.len(), 1);
    let body = body[0];

    let paragraph_selector = Selector::parse("p").unwrap();
    let mut paragraphs = body.select(&paragraph_selector);

    // TODO: Where do the authors go?
    let byline = paragraphs.next().unwrap().text().next().unwrap();
    let authors: Vec<String> = byline.strip_prefix("By")
        .unwrap_or(byline)
        .split("and")
        .flat_map(|substring| substring.split(','))
        .map(replace_html_entities)
        .collect();
    let authors: Vec<&str> = authors.iter()
        .map(|substring| substring.trim())
        .filter(|name| !name.is_empty())
        .collect();
    println!("Authors: {:?}", authors);
    
    for paragraph in paragraphs {
        println!("{:#?}", paragraph.inner_html());
        if let Some(markdown) = translate_paragraph(paragraph) {
            writeln!(file, "{}", markdown).unwrap();
        }
    }

    Ok(())
}

fn translate_paragraph(element_ref: ElementRef) -> Option<String> {
    let first_child_node = element_ref.first_child().unwrap();
    match first_child_node.value() {
        // TODO: Translating text will need to be its own function once I need to deal with <a> elements in the text
        Text(_) => Some(replace_html_entities(&element_ref.inner_html()).trim().to_string()),
        &Element(_) => translate_element(ElementRef::wrap(first_child_node).unwrap()),
        _ => panic!("Unsupported node type {:#?}", first_child_node.value()),
    }.map(|markdown| format!("\n{}", markdown))
}

fn translate_element(element: ElementRef) -> Option<String> {
    match element.value().name() {
        "a" => Some(translate_link(element)),
        "strong" => Some(translate_strong(element)),
        "br" => None,
        "img" => Some(translate_img(element)),
        _ => panic!("Unsupported element type {}", element.value().name()),
    }
}

fn translate_link(element_ref: ElementRef) -> String {
    format!("[{}]({})", replace_html_entities(&element_ref.inner_html()).trim(), element_ref.value().attr("href").unwrap())
}

fn translate_strong(element_ref: ElementRef) -> String {
    format!("**{}**", replace_html_entities(&element_ref.inner_html()).trim())
}

// TODO: Download image to file
fn translate_img(element_ref: ElementRef) -> String {
    let src = element_ref.value().attr("src").unwrap();
    let url = Url::parse(src).unwrap();
    format!("![](/assets/images{}.png){{: .img-fluid }}", url.path())
}

fn replace_html_entities(dirty_str: &str) -> String {
    dirty_str.replace("&nbsp;", " ")
}

fn create_file_path(url_str: &str) -> PathBuf {
    let url = Url::parse(url_str).unwrap();
    let url_path = url.path();
    let no_prefix = url_path.strip_prefix('/').unwrap_or(url_path);
    let no_suffix = no_prefix.strip_suffix('/').unwrap_or(no_prefix);
    let path_str = format!("{}.md", no_suffix);
    PathBuf::from(&path_str)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_path_creation() {
        let path = create_file_path("https://www.sacdsa.org/blog/2020/07/06/a-people-of-color-s-history-of-dsa-part-4-DSA-Looks-Inward/");
        assert_eq!(path.to_str(), Some("blog/2020/07/06/a-people-of-color-s-history-of-dsa-part-4-DSA-Looks-Inward.md"))
    }

    #[test]
    fn test_link_translate() {
        let raw_html_str = r#"<a href="https://www.sacdsa.org/blog/2019/08/13/a-people-of-color-s-history-of-dsa-part-1-socialism-race-and-the-formation-of-dsa/" target="_blank">&nbsp;A People of Color's History of DSA, Part 1</a>"#;
        let html = Html::parse_fragment(raw_html_str);
        let selector = Selector::parse("a").unwrap();
        let element_ref = html.select(&selector).next().unwrap();
        let markdown = translate_link(element_ref);
        assert_eq!(markdown, "[A People of Color's History of DSA, Part 1](https://www.sacdsa.org/blog/2019/08/13/a-people-of-color-s-history-of-dsa-part-1-socialism-race-and-the-formation-of-dsa/)");
    }

    #[test]
    fn test_child_conversion() {
        let raw_html_str = r#"<p><a href="https://www.sacdsa.org/blog/2019/08/13/a-people-of-color-s-history-of-dsa-part-1-socialism-race-and-the-formation-of-dsa/" target="_blank">&nbsp;A People of Color's History of DSA, Part 1</a></p>"#;
        let html = Html::parse_fragment(raw_html_str);
        let selector = Selector::parse("p").unwrap();
        let element_ref = html.select(&selector).next().unwrap();
        let markdown = translate_paragraph(element_ref);
        assert_eq!(markdown, Some("\n[A People of Color's History of DSA, Part 1](https://www.sacdsa.org/blog/2019/08/13/a-people-of-color-s-history-of-dsa-part-1-socialism-race-and-the-formation-of-dsa/)".to_string()));
    }
    
    #[test]
    fn test_bolding_strongs() {
        let raw_html_str = r#"<strong>4: DSA Looks Inward</strong>"#;
        let html = Html::parse_fragment(raw_html_str);
        let selector = Selector::parse("strong").unwrap();
        let element_ref = html.select(&selector).next().unwrap();
        let markdown = translate_strong(element_ref);
        assert_eq!(markdown, "**4: DSA Looks Inward**");
    }
    
    #[test]
    fn test_skip_brs() {
        let raw_html_str = "<br>";
        let html = Html::parse_fragment(raw_html_str);
        let selector = Selector::parse("br").unwrap();
        let element_ref = html.select(&selector).next().unwrap();
        let markdown = translate_element(element_ref);
        assert_eq!(markdown, None);
    }

    #[test]
    fn test_trim_domain_from_url() {
        let url_str = "https://lh4.googleusercontent.com/tf2qRXcS4yKnX-Z-vYYbvLuEF-xWCQXM0bK9R-KtfxrQcwjaELbULke0oUbPJMPp9EuuZ6EImm4X5ycTjQcCixAmh2E9gOFZNkcMso9h3BngaNFDuNSBpoSfbXZCLpSAZSmF3j1o";
        let url = Url::parse(url_str).unwrap();
        assert_eq!(url.path(), "/tf2qRXcS4yKnX-Z-vYYbvLuEF-xWCQXM0bK9R-KtfxrQcwjaELbULke0oUbPJMPp9EuuZ6EImm4X5ycTjQcCixAmh2E9gOFZNkcMso9h3BngaNFDuNSBpoSfbXZCLpSAZSmF3j1o")
    }

    #[test]
    fn test_img_translate() {
        let raw_html_str = r#"<img src="https://lh4.googleusercontent.com/tf2qRXcS4yKnX-Z-vYYbvLuEF-xWCQXM0bK9R-KtfxrQcwjaELbULke0oUbPJMPp9EuuZ6EImm4X5ycTjQcCixAmh2E9gOFZNkcMso9h3BngaNFDuNSBpoSfbXZCLpSAZSmF3j1o">"#;
        let html = Html::parse_fragment(raw_html_str);
        let selector = Selector::parse("img").unwrap();
        let element_ref = html.select(&selector).next().unwrap();
        let markdown = translate_img(element_ref);
        assert_eq!(markdown, "![](/assets/images/tf2qRXcS4yKnX-Z-vYYbvLuEF-xWCQXM0bK9R-KtfxrQcwjaELbULke0oUbPJMPp9EuuZ6EImm4X5ycTjQcCixAmh2E9gOFZNkcMso9h3BngaNFDuNSBpoSfbXZCLpSAZSmF3j1o.png){: .img-fluid }");
    }

    #[test]
    fn test_remove_nbsp() {
        let raw_str = r#"&nbsp;Hello&nbsp;World&nbsp;"#;
        let replaced_str = replace_html_entities(raw_str);
        let fixed_str = replaced_str.trim();
        assert_eq!(fixed_str, "Hello World");
    }
}