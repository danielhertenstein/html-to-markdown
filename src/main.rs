use reqwest::{Client, Result};
use scraper::{ElementRef, Html, Node::{Element, Text}, Selector};

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

    let title_selector = Selector::parse("h1").unwrap();
    let title: Vec<ElementRef> = content.select(&title_selector).collect();
    assert_eq!(title.len(), 1);
    let title = title[0];
    println!("Title: {}", title.inner_html().trim());

    let date_selector = Selector::parse(r#"p[class="text-light"]"#).unwrap();
    let date: Vec<ElementRef> = content.select(&date_selector).collect();
    assert_eq!(date.len(), 1);
    let date = date[0];
    println!("Date: {}", date.inner_html().trim());

    let body_selector = Selector::parse(r#"div[class="quill-output"]"#).unwrap();
    let body: Vec<ElementRef> = content.select(&body_selector).collect();
    assert_eq!(body.len(), 1);
    let body = body[0];

    let paragraph_selector = Selector::parse("p").unwrap();
    let mut paragraphs = body.select(&paragraph_selector);

    let byline = paragraphs.next().unwrap().text().next().unwrap();
    let authors: Vec<&str> = byline.strip_prefix("By")
        .unwrap_or(byline)
        .split("and")
        .flat_map(|substring| substring.split(','))
        .map(|substring| substring.trim())
        .filter(|name| !name.is_empty())
        .collect();
    println!("Authors: {:?}", authors);
    
    for paragraph in paragraphs {
        println!("{:#?}", paragraph.inner_html());
        let markdown = translate_paragraph(paragraph);
        println!("{}", markdown);
    }

    Ok(())
}

fn translate_paragraph(element_ref: ElementRef) -> String {
    let first_child_node = element_ref.first_child().unwrap();
    let inner_markdown = match first_child_node.value() {
        Text(_) => element_ref.inner_html(),
        &Element(_) => translate_element(ElementRef::wrap(first_child_node).unwrap()),
        _ => panic!("Unsupported node type {:#?}", first_child_node.value()),
    };
    format!("\n{}", inner_markdown)
}

fn translate_element(element: ElementRef) -> String {
    match element.value().name() {
        "a" => translate_link(element),
        "strong" => element.inner_html(),
        "br" => "".to_string(),
        _ => panic!("Unsupported element type {}", element.value().name()),
    }
}

fn translate_link(element_ref: ElementRef) -> String {
    format!("[{}]({})", element_ref.inner_html(), element_ref.value().attr("href").unwrap())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_link_translate() {
        let link_str = r#"<a href="https://www.sacdsa.org/blog/2019/08/13/a-people-of-color-s-history-of-dsa-part-1-socialism-race-and-the-formation-of-dsa/" target="_blank">&nbsp;A People of Color's History of DSA, Part 1</a>"#;
        let html = Html::parse_fragment(link_str);
        let selector = Selector::parse("a").unwrap();
        let link = html.select(&selector).next().unwrap();
        let markdown = translate_link(link);
        assert_eq!(markdown, "[&nbsp;A People of Color's History of DSA, Part 1](https://www.sacdsa.org/blog/2019/08/13/a-people-of-color-s-history-of-dsa-part-1-socialism-race-and-the-formation-of-dsa/)");
    }

    #[test]
    fn test_child_conversion() {
        let paragraph_str = r#"<p><a href="https://www.sacdsa.org/blog/2019/08/13/a-people-of-color-s-history-of-dsa-part-1-socialism-race-and-the-formation-of-dsa/" target="_blank">&nbsp;A People of Color's History of DSA, Part 1</a></p>"#;
        let html = Html::parse_fragment(paragraph_str);
        let selector = Selector::parse("p").unwrap();
        let paragraph = html.select(&selector).next().unwrap();
        let markdown = translate_paragraph(paragraph);
        assert_eq!(markdown, "\n[&nbsp;A People of Color's History of DSA, Part 1](https://www.sacdsa.org/blog/2019/08/13/a-people-of-color-s-history-of-dsa-part-1-socialism-race-and-the-formation-of-dsa/)");
    }
    
    #[test]
    fn test_bolding_strongs() {
        assert_eq!(true, false);
    }

    #[test]
    fn test_img_translate() {
        assert_eq!(true, false);
    }
}