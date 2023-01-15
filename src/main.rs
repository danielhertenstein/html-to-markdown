use reqwest::{Client, Result};
use scraper::{ElementRef, Html, Node::{Element, Text}, Selector};
use std::{fs::File, io::Write};
use std::path::{Path, PathBuf};
use url::Url;

const DOMAIN: &str = "www.sacdsa.org";

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::new();
    let pages: Vec<&str> = [
        // "https://www.sacdsa.org/blog/2022/08/24/gaza-killing-fields-open-and-shut-quickly-why-and-how-to-stop-the-carnage/",
        // "https://www.sacdsa.org/blog/2020/10/06/prop22-discontents/",
        // "https://www.sacdsa.org/blog/2020/09/16/not-an-empty-round-a-response-to-objections-by-the-sacramento-dsa-cpn-caucus-on-resolution-9/",
        // "https://www.sacdsa.org/blog/2020/09/10/the-time-is-now/",
        // "https://www.sacdsa.org/blog/2020/07/25/thoughts-on-organizing-to-keep-schools-safe/",
        // "https://www.sacdsa.org/blog/2020/07/06/a-people-of-color-s-history-of-dsa-part-4-DSA-Looks-Inward/",
        // "https://www.sacdsa.org/blog/2020/06/01/george-floyd-solidarity-statement/",
        // "https://www.sacdsa.org/blog/2020/05/11/free-support-healthcare-workers-poster/",
        // "https://www.sacdsa.org/blog/2020/04/30/chapter-statement-on-covid-crisis/",
        // "https://www.sacdsa.org/blog/2020/03/10/sacramento-democratic-socialists-win-first-seat-on-city-council/",
        // "https://www.sacdsa.org/blog/2019/12/09/sacramento-s-rent-control-fight-is-about-power-not-process/",
        // "https://www.sacdsa.org/blog/2019/12/04/a-people-of-color-s-history-of-dsa-part-3-dsa-and-the-first-rainbow-coalition/",
        // "https://www.sacdsa.org/blog/2019/10/22/beyond-nonprofits-toward-change/",
        // "https://www.sacdsa.org/blog/2019/09/11/a-people-of-color-s-history-of-dsa-part-2-dsa-enters-the-80s/",
        // "https://www.sacdsa.org/blog/2019/08/13/a-people-of-color-s-history-of-dsa-part-1-socialism-race-and-the-formation-of-dsa/",
        // "https://www.sacdsa.org/blog/2019/07/26/against-resolution-22/",
        // "https://www.sacdsa.org/blog/2019/07/26/dont-trust-denney/",
        // "https://www.sacdsa.org/blog/2019/05/15/socialized_sac_ep1/",
        // "https://www.sacdsa.org/blog/2019/05/11/will_california_support_unions/",
        // "https://www.sacdsa.org/blog/2019/03/30/thoughts_on_m4a_canvassing/",
        // "https://www.sacdsa.org/blog/2019/03/18/racial_solidarity_committee_mission_statement/",
        // "https://www.sacdsa.org/blog/2019/03/14/democratic_socialist_for_mayor_2020/",
        // "https://www.sacdsa.org/blog/2019/03/12/op_ed_reorganization_sac_dsa/",
        // "https://www.sacdsa.org/blog/2019/03/04/statement_stephon_clark_no_charges/",
        "https://www.sacdsa.org/blog/2019/01/30/venezuela_solidarity_statement_01_30_19/",
        "https://www.sacdsa.org/blog/2019/01/30/green_new_deal_pge_kickoff/",
        "https://www.sacdsa.org/blog/2019/01/06/international-committee-mission-statement/",
        "https://www.sacdsa.org/blog/2018/10/23/research_nov_2018_voting_guide/",
        "https://www.sacdsa.org/blog/2018/10/05/sac_dsa_kavanaugh_statement/",
        "https://www.sacdsa.org/blog/2018/08/15/socialist_support_2018_national_prison_strike/",
        "https://www.sacdsa.org/blog/2018/07/28/sac_city_council/",
        "https://www.sacdsa.org/blog/2018/07/23/immigration_and_labor_in_ca/",
        "https://www.sacdsa.org/blog/2018/06/16/tentsarentenough/",
        "https://www.sacdsa.org/blog/2018/05/15/sacramento_dsa_endorsements/",
        "https://www.sacdsa.org/blog/2018/05/07/ucstrike/",
        "https://www.sacdsa.org/blog/2018/04/04/not-one-more-statement-on-stephon-clark/",
        "https://www.sacdsa.org/blog/2018/04/03/forrentcontrol/",
        "https://www.sacdsa.org/blog/2018/03/28/sb827/",
        "https://www.sacdsa.org/blog/2018/03/20/bera/",
        "https://www.sacdsa.org/blog/2018/03/08/what_is_going_on_with_labor_unions/",
        "https://www.sacdsa.org/blog/2018/01/26/turkish_aggression_against_afrin_rojava_solidarity/",
        "https://www.sacdsa.org/blog/2018/01/25/interview_michael_lighty_the_fight_for_universal_healthcare_in_california/",
        "https://www.sacdsa.org/blog/2018/01/25/fred_glass_california_labor_in_the_time_of_trump/",
        "https://www.sacdsa.org/blog/2018/01/12/solidarity_with_the_rohingya/",
        "https://www.sacdsa.org/blog/2018/01/10/costa_hawkins_and_why_it_matters/",
        "https://www.sacdsa.org/blog/2018/01/03/rent_control_opponents_heavily_funding_sacramento_politicians/",
        "https://www.sacdsa.org/blog/2017/12/11/the_approaching_battle_for_rent_control/",
        "https://www.sacdsa.org/blog/2017/11/25/statement_in_remembrance_of_michael_israel/",
        "https://www.sacdsa.org/blog/2017/11/20/what-we-can-do/",
        "https://www.sacdsa.org/blog/2017/11/13/pay-your-dues/",
        "https://www.sacdsa.org/blog/2017/11/02/rentcontrol/",
        "https://www.sacdsa.org/blog/2017/10/04/landpark/",
        "https://www.sacdsa.org/blog/2017/09/22/2017election/"
    ].to_vec();
    for page in pages {
        translate_site(&client, page).await?;
    }

    Ok(())
}

async fn translate_site(client: &Client, url: &str) -> Result<()> {
    dbg!(&url);
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
    let mut file = create_file(&path);

    // Start of liquid header
    writeln!(file, "---\nlayout: page").unwrap();

    let title_selector = Selector::parse("h1").unwrap();
    let title: Vec<ElementRef> = content.select(&title_selector).collect();
    assert_eq!(title.len(), 1);
    let title = title[0];
    writeln!(file, "title: \"{}\"", replace_html_entities(&title.inner_html()).trim()).unwrap();

    // TODO: Where does the date go?
    // let date_selector = Selector::parse(r#"p[class="text-light"]"#).unwrap();
    // let date: Vec<ElementRef> = content.select(&date_selector).collect();
    // assert_eq!(date.len(), 1);
    // let date = date[0];
    // println!("Date: {}", replace_html_entities(&date.inner_html()).trim());

    let body_selector = Selector::parse(r#"div[class="quill-output"]"#).unwrap();
    let body: Vec<ElementRef> = content.select(&body_selector).collect();
    assert_eq!(body.len(), 1);
    let body = body[0];

    let image_selector = Selector::parse("img").unwrap();
    let images = body.select(&image_selector);
    for image in images {
        let url = url_from_img(image);
        download_image(url, PathBuf::from("assets/images"), client).await?;
    }

    let paragraph_selector = Selector::parse("p").unwrap();
    let mut paragraphs = body.select(&paragraph_selector);

    // TODO: When we support multiple authors, this code will be more useful
    let byline = paragraphs.next().unwrap().text().next().unwrap();
    let authors = byline.strip_prefix("By").unwrap_or(byline);
    writeln!(file, "author: {}", authors).unwrap();
    // let authors: Vec<String> = byline.strip_prefix("By")
    //     .unwrap_or(byline)
    //     .split("and")
    //     .flat_map(|substring| substring.split(','))
    //     .map(replace_html_entities)
    //     .collect();
    // let authors: Vec<&str> = authors.iter()
    //     .map(|substring| substring.trim())
    //     .filter(|name| !name.is_empty())
    //     .collect();
    // println!("author: {:?}", authors);
    
    // End of liquid header
    writeln!(file, "permalink: /{}/\n---", path.with_extension("").display()).unwrap();

    for paragraph in paragraphs {
        println!("{:#?}", paragraph.inner_html());
        if let Some(markdown) = translate_paragraph(paragraph) {
            writeln!(file, "\n{}", markdown).unwrap();
        }
    }

    Ok(())
}

fn translate_paragraph(element_ref: ElementRef) -> Option<String> {
    // TODO: Probably need to iterate over all children for the general case
    let first_child_node = element_ref.first_child().unwrap();
    match first_child_node.value() {
        Text(_) => Some(translate_text(element_ref)),
        &Element(_) => translate_element(ElementRef::wrap(first_child_node).unwrap()),
        _ => panic!("Unsupported node type {:#?}", first_child_node.value()),
    }
}

fn translate_element(element: ElementRef) -> Option<String> {
    match element.value().name() {
        "a" => Some(translate_link(element)),
        "strong" => Some(translate_strong(element)),
        "em" => Some(translate_em(element)),
        "u" => Some(translate_u(element)),
        "sup" => Some(translate_sup(element)),
        "br" => None,
        "img" => Some(translate_img(element)),
        "span" => translate_span(element),
        _ => panic!("Unsupported element type {}", element.value().name()),
    }
}

fn translate_text(element: ElementRef) -> String {
    element
        .children()
        .filter_map(|node| {
            match node.value() {
                Text(text) => Some(replace_html_entities(text).trim().to_string()),
                Element(_) => translate_element(ElementRef::wrap(node).unwrap()),
                _ => panic!("Unsupported node type {:#?}", node.value()),
            }
        })
        .fold(String::new(), |mut a, b| {
            if a.ends_with(' ') && b.starts_with(',') {
                a.pop();
            }
            a.reserve(b.len() + 1);
            a.push_str(&b);
            a.push(' ');
            a
        })
        .trim_end()
        .to_string()
}

fn translate_link(element_ref: ElementRef) -> String {
    let href = element_ref.value().attr("href").unwrap();
    let url = Url::parse(href).unwrap();
    let markdown_url = match url.domain() {
        Some(domain) if domain == DOMAIN => url.path(),
        _ => url.as_str()
    };
    format!("[{}]({})", replace_html_entities(&element_ref.inner_html()).trim(), markdown_url)
}

fn translate_strong(element_ref: ElementRef) -> String {
    format!("**{}**", translate_paragraph(element_ref).unwrap())
}

fn translate_em(element_ref: ElementRef) -> String {
    format!("*{}*", translate_paragraph(element_ref).unwrap())
}

fn translate_u(element_ref: ElementRef) -> String {
    format!("_{}_", translate_paragraph(element_ref).unwrap())
}

fn translate_sup(element_ref: ElementRef) -> String {
    format!("<sup>{}</sup>", translate_paragraph(element_ref).unwrap())
}

fn translate_span(element_ref: ElementRef) -> Option<String> {
    translate_paragraph(element_ref)
}

fn translate_img(element_ref: ElementRef) -> String {
    let url = url_from_img(element_ref);
    if url.scheme() == "data" {
        return "**There is a base64 image here that I don't support yet**.".to_string();
    }
    let mut filepath = Path::new("/assets/images").join(url.path().strip_prefix('/').unwrap());
    if filepath.as_path().extension().is_none() {
        filepath.set_extension("png");
    }
    let markdown = format!("![]({}){{: .img-fluid }}", filepath.display());
    markdown
}

fn url_from_img(element_ref: ElementRef) -> Url {
    let src = element_ref.value().attr("src").unwrap();
    Url::parse(src).unwrap()
}

async fn download_image(url: Url, directory: PathBuf, client: &Client) -> Result<()> {
    if url.scheme() == "data" {
        println!("Skipping base64 image download for now.");
        return Ok(());
    }
    let filename = format!("{}.png", url.path().strip_prefix('/').unwrap());
    let path = directory.join(filename);
    let mut file = create_file(&path);
    let image_bytes = client.get(url)
        .send()
        .await?
        .bytes()
        .await?;
    file.write_all(&image_bytes).unwrap();
    Ok(())
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

fn create_file(path: &PathBuf) -> File {
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).unwrap();
    match File::create(path) {
        Err(why) => panic!("Couldn't create {}: {}", path.display(), why),
        Ok(file) => file,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempdir::TempDir;

    #[test]
    fn test_file_path_creation() {
        let path = create_file_path("https://www.sacdsa.org/blog/2020/07/06/a-people-of-color-s-history-of-dsa-part-4-DSA-Looks-Inward/");
        assert_eq!(path.to_str(), Some("blog/2020/07/06/a-people-of-color-s-history-of-dsa-part-4-DSA-Looks-Inward.md"))
    }

    #[test]
    fn test_link_translate_different_domain() {
        let raw_html_str = r#"<a href="https://www.fake_site.org/some_link">Link Text</a>"#;
        let html = Html::parse_fragment(raw_html_str);
        let selector = Selector::parse("a").unwrap();
        let element_ref = html.select(&selector).next().unwrap();
        let markdown = translate_link(element_ref);
        assert_eq!(markdown, "[Link Text](https://www.fake_site.org/some_link)");
    }

    #[test]
    fn test_child_conversion() {
        let raw_html_str = r#"<p><a href="https://www.sacdsa.org/blog/2019/08/13/a-people-of-color-s-history-of-dsa-part-1-socialism-race-and-the-formation-of-dsa/" target="_blank">&nbsp;A People of Color's History of DSA, Part 1</a></p>"#;
        let html = Html::parse_fragment(raw_html_str);
        let selector = Selector::parse("p").unwrap();
        let element_ref = html.select(&selector).next().unwrap();
        let markdown = translate_paragraph(element_ref);
        assert_eq!(markdown, Some("[A People of Color's History of DSA, Part 1](/blog/2019/08/13/a-people-of-color-s-history-of-dsa-part-1-socialism-race-and-the-formation-of-dsa/)".to_string()));
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
    fn test_italics_em() {
        let raw_html_str = r#"<em>4: DSA Looks Inward</em>"#;
        let html = Html::parse_fragment(raw_html_str);
        let selector = Selector::parse("em").unwrap();
        let element_ref = html.select(&selector).next().unwrap();
        let markdown = translate_em(element_ref);
        assert_eq!(markdown, "*4: DSA Looks Inward*");
    }

    #[test]
    fn test_superscripting_sup() {
        let raw_html_str = r#"<sup>4</sup>"#;
        let html = Html::parse_fragment(raw_html_str);
        let selector = Selector::parse("sup").unwrap();
        let element_ref = html.select(&selector).next().unwrap();
        let markdown = translate_sup(element_ref);
        assert_eq!(markdown, "<sup>4</sup>");
    }

    #[test]
    fn test_underlining_u() {
        let raw_html_str = r#"<u>4: DSA Looks Inward</u>"#;
        let html = Html::parse_fragment(raw_html_str);
        let selector = Selector::parse("u").unwrap();
        let element_ref = html.select(&selector).next().unwrap();
        let markdown = translate_u(element_ref);
        assert_eq!(markdown, "_4: DSA Looks Inward_");
    }

    #[test]
    fn test_passing_through_span() {
        let raw_html_str = r#"<span>4: DSA Looks Inward</span>"#;
        let html = Html::parse_fragment(raw_html_str);
        let selector = Selector::parse("span").unwrap();
        let element_ref = html.select(&selector).next().unwrap();
        let markdown = translate_span(element_ref);
        assert_eq!(markdown, Some("4: DSA Looks Inward".to_string()));
    }

    #[test]
    fn test_span_with_only_br() {
        let raw_html_str = r#"<span><br></span>"#;
        let html = Html::parse_fragment(raw_html_str);
        let selector = Selector::parse("span").unwrap();
        let element_ref = html.select(&selector).next().unwrap();
        let markdown = translate_span(element_ref);
        assert_eq!(markdown, None);
    }
    
    #[test]
    fn test_strong_with_img_inside() {
        let raw_html_str = r#"<strong><img src="https://www.fake_site.com/fake_image.png"></strong>"#;
        let html = Html::parse_fragment(raw_html_str);
        let selector = Selector::parse("strong").unwrap();
        let element_ref = html.select(&selector).next().unwrap();
        let markdown = translate_strong(element_ref);
        assert_eq!(markdown, "**![](/assets/images/fake_image.png){: .img-fluid }**");
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

    #[test]
    fn test_translate_link_inside_text() {
        let raw_html_str = r#"<p>There is some text, <a href="https://fake_site.com/fake_page.html">then a link</a>, and then more text."#;
        let html = Html::parse_fragment(raw_html_str);
        let selector = Selector::parse("p").unwrap();
        let element_ref = html.select(&selector).next().unwrap();
        let markdown = translate_paragraph(element_ref);
        assert_eq!(markdown, Some("There is some text, [then a link](https://fake_site.com/fake_page.html), and then more text.".to_string()));
    }

    #[tokio::test]
    async fn test_download_img_with_full_url_src() {
        let raw_html_str = r#"<img src="https://lh4.googleusercontent.com/tf2qRXcS4yKnX-Z-vYYbvLuEF-xWCQXM0bK9R-KtfxrQcwjaELbULke0oUbPJMPp9EuuZ6EImm4X5ycTjQcCixAmh2E9gOFZNkcMso9h3BngaNFDuNSBpoSfbXZCLpSAZSmF3j1o">"#;
        let html = Html::parse_fragment(raw_html_str);
        let selector = Selector::parse("img").unwrap();
        let element_ref = html.select(&selector).next().unwrap();
        let url = url_from_img(element_ref);
        let tmp_dir = TempDir::new("testing_dir").unwrap();
        let client = Client::new();
        download_image(url, tmp_dir.path().to_path_buf(), &client).await.unwrap();
        assert!(tmp_dir.path().join("tf2qRXcS4yKnX-Z-vYYbvLuEF-xWCQXM0bK9R-KtfxrQcwjaELbULke0oUbPJMPp9EuuZ6EImm4X5ycTjQcCixAmh2E9gOFZNkcMso9h3BngaNFDuNSBpoSfbXZCLpSAZSmF3j1o.png").exists());
    }

    #[test]
    fn test_link_matching_domain() {
        let raw_html_str = r#"<a href="https://www.sacdsa.org/some_link">Link Text</a>"#;
        let html = Html::parse_fragment(raw_html_str);
        let selector = Selector::parse("a").unwrap();
        let element_ref = html.select(&selector).next().unwrap();
        let markdown = translate_link(element_ref);
        assert_eq!(markdown, "[Link Text](/some_link)");
    }
}