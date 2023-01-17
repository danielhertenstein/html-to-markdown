use ego_tree::NodeRef;
use reqwest::{Client, Result};
use scraper::{
    ElementRef, Html,
    Node::{self, Element, Text},
    Selector,
};
use std::path::{Path, PathBuf};
use std::{fs::{File, OpenOptions}, io::Write};
use url::Url;

const DOMAIN: &str = "www.sacdsa.org";

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::new();
    let pages: Vec<&str> = [
        "https://www.sacdsa.org/blog/2022/08/24/gaza-killing-fields-open-and-shut-quickly-why-and-how-to-stop-the-carnage/",
        "https://www.sacdsa.org/blog/2020/10/06/prop22-discontents/",
        "https://www.sacdsa.org/blog/2020/09/16/not-an-empty-round-a-response-to-objections-by-the-sacramento-dsa-cpn-caucus-on-resolution-9/",
        "https://www.sacdsa.org/blog/2020/09/10/the-time-is-now/",
        "https://www.sacdsa.org/blog/2020/07/25/thoughts-on-organizing-to-keep-schools-safe/",
        "https://www.sacdsa.org/blog/2020/07/06/a-people-of-color-s-history-of-dsa-part-4-DSA-Looks-Inward/",
        "https://www.sacdsa.org/blog/2020/06/01/george-floyd-solidarity-statement/",
        "https://www.sacdsa.org/blog/2020/05/11/free-support-healthcare-workers-poster/",
        "https://www.sacdsa.org/blog/2020/04/30/chapter-statement-on-covid-crisis/",
        "https://www.sacdsa.org/blog/2020/03/10/sacramento-democratic-socialists-win-first-seat-on-city-council/",
        "https://www.sacdsa.org/blog/2019/12/09/sacramento-s-rent-control-fight-is-about-power-not-process/",
        "https://www.sacdsa.org/blog/2019/12/04/a-people-of-color-s-history-of-dsa-part-3-dsa-and-the-first-rainbow-coalition/",
        "https://www.sacdsa.org/blog/2019/10/22/beyond-nonprofits-toward-change/",
        "https://www.sacdsa.org/blog/2019/09/11/a-people-of-color-s-history-of-dsa-part-2-dsa-enters-the-80s/",
        "https://www.sacdsa.org/blog/2019/08/13/a-people-of-color-s-history-of-dsa-part-1-socialism-race-and-the-formation-of-dsa/",
        "https://www.sacdsa.org/blog/2019/07/26/against-resolution-22/",
        "https://www.sacdsa.org/blog/2019/07/26/dont-trust-denney/",
        "https://www.sacdsa.org/blog/2019/05/15/socialized_sac_ep1/",
        "https://www.sacdsa.org/blog/2019/05/11/will_california_support_unions/",
        "https://www.sacdsa.org/blog/2019/03/30/thoughts_on_m4a_canvassing/",
        "https://www.sacdsa.org/blog/2019/03/18/racial_solidarity_committee_mission_statement/",
        "https://www.sacdsa.org/blog/2019/03/14/democratic_socialist_for_mayor_2020/",
        "https://www.sacdsa.org/blog/2019/03/12/op_ed_reorganization_sac_dsa/",
        "https://www.sacdsa.org/blog/2019/03/04/statement_stephon_clark_no_charges/",
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
    let raw_html = client.get(url).send().await?.text().await?;

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
    // We just assume that the first <h1> element is the title
    let title = title[0];
    writeln!(
        file,
        "title: \"{}\"",
        replace_html_entities(&title.inner_html()).trim()
    )
    .unwrap();

    // End of liquid header
    writeln!(
        file,
        "permalink: /{}/\n---",
        path.with_extension("").display()
    )
    .unwrap();

    let date_selector = Selector::parse(r#"p[class="text-light"]"#).unwrap();
    let date: Vec<ElementRef> = content.select(&date_selector).collect();
    assert_eq!(date.len(), 1);
    let date = date[0];
    writeln!(file, "{}", replace_html_entities(&date.inner_html()).trim()).unwrap();

    // Handle the image at the top if there is one
    let featured_image_selector = Selector::parse(r#"img[class="news-featured-image"]"#).unwrap();
    let featured_image = document.select(&featured_image_selector).next();
    if let Some(image) = featured_image {
        if let Some(url) = url_from_img(image) {
            download_image(url, PathBuf::from("assets/images"), client).await?;
        }
        if let Some(markdown) = translate_element(image) {
            writeln!(file, "\n{}", markdown).unwrap();
        }
    }

    let body_selector = Selector::parse(r#"div[class="quill-output"]"#).unwrap();
    let body: Vec<ElementRef> = content.select(&body_selector).collect();
    assert_eq!(body.len(), 1);
    let body = body[0];

    let image_selector = Selector::parse("img").unwrap();
    let images = body.select(&image_selector);
    for image in images {
        if let Some(url) = url_from_img(image) {
            download_image(url, PathBuf::from("assets/images"), client).await?;
        }
    }

    body.children()
        .filter_map(translate_node)
        .for_each(|markdown| writeln!(file, "\n{}", markdown).unwrap());

    // TODO: How to handle potentially no author
    // TODO: When we support multiple authors, this code will be more useful
    // let byline = paragraphs.next().unwrap().text().next().unwrap();
    // let authors = byline.strip_prefix("By").unwrap_or(byline);
    // writeln!(file, "author: {}", authors).unwrap();
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

    Ok(())
}

fn translate_container(element: ElementRef) -> Option<String> {
    let markdown = element
        .children()
        .filter_map(translate_node)
        .collect::<Vec<String>>()
        .join("\n\n");
    (!markdown.is_empty()).then_some(markdown)
}

fn translate_node(node: NodeRef<Node>) -> Option<String> {
    match node.value() {
        Text(text) => Some(replace_html_entities(text).trim().to_string()),
        Element(_) => translate_element(ElementRef::wrap(node).unwrap()),
        _ => panic!("Unsupported node type {:#?}", node.value()),
    }
}

fn translate_element(element: ElementRef) -> Option<String> {
    println!("{}", &element.html());
    match element.value().name() {
        "a" => Some(translate_link(element)),
        "blockquote" => translate_and_wrap(element, Some("< "), None),
        "br" => None,
        "div" => translate_container(element),
        "em" => translate_and_wrap(element, Some("*"), Some("*")),
        "hr" => Some("---".to_string()),
        "h1" => translate_and_wrap(element, Some("# "), None),
        "h2" => translate_and_wrap(element, Some("## "), None),
        "h3" => translate_and_wrap(element, Some("### "), None),
        "h4" => translate_and_wrap(element, Some("#### "), None),
        "img" => Some(translate_img(element)),
        "li" => translate_text(element),
        "ol" => translate_ol(element),
        "p" => translate_text(element),
        "span" => translate_container(element),
        "strong" => translate_and_wrap(element, Some("**"), Some("**")),
        "sup" => translate_and_wrap(element, Some("<sup>"), Some("</sup>")),
        "table" => Some(element.html()),
        "u" => translate_and_wrap(element, Some("<u>"), Some("<u>")),
        "ul" => translate_ul(element),
        _ => panic!("Unsupported element type {}", element.value().name()),
    }
}

fn translate_text(element: ElementRef) -> Option<String> {
    let text = element
        .children()
        .filter_map(translate_node)
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
        .to_string();
    (!text.is_empty()).then_some(text)
}

fn translate_link(element_ref: ElementRef) -> String {
    let mut href = element_ref.value().attr("href").unwrap().to_string();
    if !href.starts_with("https://") && !href.starts_with("http://") {
        href.insert_str(0, "https://");
    }
    let url = Url::parse(&href).unwrap();
    let markdown_url = match url.domain() {
        Some(domain) if domain == DOMAIN => url.path(),
        _ => url.as_str(),
    };
    format!(
        "[{}]({})",
        replace_html_entities(&element_ref.inner_html()).trim(),
        markdown_url
    )
}

fn translate_and_wrap(
    element_ref: ElementRef,
    prefix: Option<&str>,
    suffix: Option<&str>,
) -> Option<String> {
    translate_text(element_ref).map(|mut markdown| {
        if let Some(prefix) = prefix {
            markdown.insert_str(0, prefix);
        }
        if let Some(suffix) = suffix {
            markdown.push_str(suffix);
        }
        markdown
    })
}

fn translate_ul(element_ref: ElementRef) -> Option<String> {
    let markdown = element_ref
        .children()
        .filter_map(translate_node)
        .map(|markdown| format!("* {}", markdown))
        .collect::<Vec<String>>()
        .join("\n");
    (!markdown.is_empty()).then_some(markdown)
}

fn translate_ol(element_ref: ElementRef) -> Option<String> {
    let markdown = element_ref
        .children()
        .filter_map(translate_node)
        .enumerate()
        .map(|(index, markdown)| format!("{}. {}", index + 1, markdown))
        .collect::<Vec<String>>()
        .join("\n");
    (!markdown.is_empty()).then_some(markdown)
}

fn translate_img(element_ref: ElementRef) -> String {
    match url_from_img(element_ref) {
        Some(url) => {
            if url.scheme() == "data" {
                return "**There is a base64 image here that I don't support yet**.".to_string();
            }
            let filepath =
                Path::new("/assets/images").join(filename_from_url(&url));
            // if filepath.as_path().extension().is_none() {
            //     filepath.set_extension("png");
            // }
            format!("![]({}){{: .img-fluid }}", filepath.display())
        }
        None => format!(
            "![]({}){{: .img-fluid }}",
            element_ref.value().attr("src").unwrap()
        ),
    }
}

fn url_from_img(element_ref: ElementRef) -> Option<Url> {
    let mut src = element_ref.value().attr("src").unwrap().to_string();
    if src.starts_with('/') {
        src.insert_str(0, &("https://".to_owned() + DOMAIN));
    }
    // TODO: Return the actual `Result` when I understand error handling better
    Url::parse(&src).ok()
}

fn filename_from_url(url: &Url) -> String {
    match url.path_segments() {
        Some(segments) => segments.collect::<Vec<&str>>().join("_"),
        None => panic!("Url {} has no path fragments. This shouldn't be a valid path to an image", url),
    }
}

async fn download_image(url: Url, directory: PathBuf, client: &Client) -> Result<()> {
    if url.scheme() == "data" {
        println!("Skipping base64 image download for now.");
        return Ok(());
    }
    let path = directory.join(filename_from_url(&url));
    // if path.as_path().extension().is_none() {
    //     path.set_extension("png");
    // }
    let mut file = create_file(&path);
    let image_bytes = client.get(url).send().await?.bytes().await?;
    file.write_all(&image_bytes).unwrap();
    Ok(())
}

fn replace_html_entities(dirty_str: &str) -> String {
    dirty_str.replace("&nbsp;", " ")
}

fn create_file_path(url_str: &str) -> PathBuf {
    let url = Url::parse(url_str).unwrap();
    let mut url_path = url.path_segments().unwrap();
    url_path.next();  // This is "blog"
    let year = url_path.next().unwrap();
    let month = url_path.next().unwrap();
    let day = url_path.next().unwrap();
    let post_name = url_path.next().unwrap();
    let path_str = format!("blog/{}/{}-{}-{}-{}.md", year, year, month, day, post_name);
    PathBuf::from(&path_str)
}

fn create_file(path: &PathBuf) -> File {
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).unwrap();
    match OpenOptions::new().write(true).create_new(true).open(path) {
        Err(why) => panic!("Couldn't create {}: {}", path.display(), why),
        Ok(file) => file,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempdir::TempDir;

    fn translate_fragment(raw_html_str: &str, selector_str: &str) -> Option<String> {
        let html = Html::parse_fragment(raw_html_str);
        let selector = Selector::parse(selector_str).unwrap();
        let element_ref = html.select(&selector).next().unwrap();
        translate_element(element_ref)
    }

    #[test]
    fn test_file_path_creation() {
        let path = create_file_path("https://www.sacdsa.org/blog/2020/07/06/a-people-of-color-s-history-of-dsa-part-4-DSA-Looks-Inward/");
        assert_eq!(
            path.to_str(),
            Some("blog/2020/2020-07-06-a-people-of-color-s-history-of-dsa-part-4-DSA-Looks-Inward.md")
        )
    }

    #[test]
    fn test_link_translate_different_domain() {
        assert_eq!(
            translate_fragment(
                r#"<a href="https://www.fake_site.org/some_link">Link Text</a>"#,
                "a"
            ),
            Some("[Link Text](https://www.fake_site.org/some_link)".to_string())
        );
    }

    #[test]
    fn test_element_in_p() {
        let markdown = translate_fragment(
            r#"<p><a href="https://www.sacdsa.org/blog/2019/08/13/a-people-of-color-s-history-of-dsa-part-1-socialism-race-and-the-formation-of-dsa/" target="_blank">&nbsp;A People of Color's History of DSA, Part 1</a></p>"#,
            "p",
        );
        assert_eq!(markdown, Some("[A People of Color's History of DSA, Part 1](/blog/2019/08/13/a-people-of-color-s-history-of-dsa-part-1-socialism-race-and-the-formation-of-dsa/)".to_string()));
    }

    #[test]
    fn test_bolding_strongs() {
        let markdown = translate_fragment(r#"<strong>4: DSA Looks Inward</strong>"#, "strong");
        assert_eq!(markdown, Some("**4: DSA Looks Inward**".to_string()));
    }

    #[test]
    fn test_italics_em() {
        let markdown = translate_fragment(r#"<em>4: DSA Looks Inward</em>"#, "em");
        assert_eq!(markdown, Some("*4: DSA Looks Inward*".to_string()));
    }

    #[test]
    fn test_superscripting_sup() {
        let markdown = translate_fragment(r#"<sup>4</sup>"#, "sup");
        assert_eq!(markdown, Some("<sup>4</sup>".to_string()));
    }

    #[test]
    fn test_underlining_u() {
        let markdown = translate_fragment(r#"<u>4: DSA Looks Inward</u>"#, "u");
        assert_eq!(markdown, Some("_4: DSA Looks Inward_".to_string()));
    }

    #[test]
    fn test_passing_through_span() {
        let markdown = translate_fragment(r#"<span>4: DSA Looks Inward</span>"#, "span");
        assert_eq!(markdown, Some("4: DSA Looks Inward".to_string()));
    }

    #[test]
    fn test_span_with_only_br() {
        let markdown = translate_fragment(r#"<span><br></span>"#, "span");
        assert_eq!(markdown, None);
    }

    #[test]
    fn test_strong_with_element_inside() {
        let markdown = translate_fragment(
            r#"<strong><img src="https://www.fake_site.com/fake_image.png"></strong>"#,
            "strong",
        );
        assert_eq!(
            markdown,
            Some("**![](/assets/images/fake_image.png){: .img-fluid }**".to_string())
        );
    }

    #[test]
    fn test_skip_brs() {
        let markdown = translate_fragment("<br>", "br");
        assert_eq!(markdown, None);
    }

    #[test]
    fn test_trim_domain_from_url() {
        let url_str = "https://lh4.googleusercontent.com/tf2qRXcS4yKnX-Z-vYYbvLuEF-xWCQXM0bK9R-KtfxrQcwjaELbULke0oUbPJMPp9EuuZ6EImm4X5ycTjQcCixAmh2E9gOFZNkcMso9h3BngaNFDuNSBpoSfbXZCLpSAZSmF3j1o";
        let url = Url::parse(url_str).unwrap();
        assert_eq!(url.path(), "/tf2qRXcS4yKnX-Z-vYYbvLuEF-xWCQXM0bK9R-KtfxrQcwjaELbULke0oUbPJMPp9EuuZ6EImm4X5ycTjQcCixAmh2E9gOFZNkcMso9h3BngaNFDuNSBpoSfbXZCLpSAZSmF3j1o")
    }

    #[test]
    fn test_img_translate_one_fragment() {
        let markdown = translate_fragment(
            r#"<img src="https://www.fake_site.org/image_src">"#,
            "img",
        );
        assert_eq!(markdown, Some("![](/assets/images/image_src){: .img-fluid }".to_string()));
    }

    #[test]
    fn test_img_translate_two_fragments() {
        let markdown = translate_fragment(
            r#"<img src="https://www.fake_site.org/image_src/another_fragment">"#,
            "img",
        );
        assert_eq!(markdown, Some("![](/assets/images/image_src_another_fragment){: .img-fluid }".to_string()));
    }

    #[test]
    fn test_img_translate_relative_src() {
        let markdown = translate_fragment(
            r#"<img src="/image_src/another_fragment">"#,
            "img",
        );
        assert_eq!(markdown, Some("![](/assets/images/image_src_another_fragment){: .img-fluid }".to_string()));
    }

    #[test]
    fn test_remove_nbsp() {
        let raw_str = r#"&nbsp;Hello&nbsp;World&nbsp;"#;
        let replaced_str = replace_html_entities(raw_str);
        let fixed_str = replaced_str.trim();
        assert_eq!(fixed_str, "Hello World");
    }

    #[test]
    fn test_translate_inline_link() {
        let markdown = translate_fragment(
            r#"<p>There is some text, <a href="https://fake_site.com/fake_page.html">then a link</a>, and then more text."#,
            "p",
        );
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
        download_image(url.unwrap(), tmp_dir.path().to_path_buf(), &client)
            .await
            .unwrap();
        assert!(tmp_dir.path().join("tf2qRXcS4yKnX-Z-vYYbvLuEF-xWCQXM0bK9R-KtfxrQcwjaELbULke0oUbPJMPp9EuuZ6EImm4X5ycTjQcCixAmh2E9gOFZNkcMso9h3BngaNFDuNSBpoSfbXZCLpSAZSmF3j1o").exists());
    }

    #[test]
    fn test_image_src_not_a_url() {
        let raw_html_str = r#"<img src="some_img.png">"#;
        let html = Html::parse_fragment(raw_html_str);
        let selector = Selector::parse("img").unwrap();
        let element_ref = html.select(&selector).next().unwrap();
        let url = url_from_img(element_ref);
        assert_eq!(url, None);
    }

    #[test]
    fn test_link_matching_domain() {
        let markdown = translate_fragment(
            r#"<a href="https://www.sacdsa.org/some_link">Link Text</a>"#,
            "a",
        );
        assert_eq!(markdown, Some("[Link Text](/some_link)".to_string()));
    }

    #[test]
    fn test_translate_link_without_www() {
        let markdown =
            translate_fragment(r#"<a href="some_site.org/some_link">Link Text</a>"#, "a");
        assert_eq!(
            markdown,
            Some("[Link Text](https://some_site.org/some_link)".to_string())
        );
    }

    #[test]
    fn test_translate_ul_with_items() {
        let markdown = translate_fragment(r#"<ul><li>One</li><li>Two</li></ul>"#, "ul");
        assert_eq!(markdown, Some("* One\n* Two".to_string()));
    }

    #[test]
    fn test_translate_ul_empty() {
        let markdown = translate_fragment(r#"<ul></ul>"#, "ul");
        assert_eq!(markdown, None);
    }

    #[test]
    fn test_translate_blockquote() {
        let markdown = translate_fragment(
            r#"<blockquote>This is a blockquote.</blockquote>"#,
            "blockquote",
        );
        assert_eq!(markdown, Some("< This is a blockquote.".to_string()));
    }

    #[test]
    fn test_translate_ol_with_items() {
        let markdown = translate_fragment(r#"<ol><li>One</li><li>Two</li></ol>"#, "ol");
        assert_eq!(markdown, Some("1. One\n2. Two".to_string()));
    }

    #[test]
    fn test_translate_ol_empty() {
        let markdown = translate_fragment(r#"<ol></ol>"#, "ol");
        assert_eq!(markdown, None);
    }

    #[test]
    fn test_translate_h1() {
        let markdown = translate_fragment(r#"<h1>Header 1</h1>"#, "h1");
        assert_eq!(markdown, Some("# Header 1".to_string()));
    }

    #[test]
    fn test_translate_h2() {
        let markdown = translate_fragment(r#"<h2>Header 2</h2>"#, "h2");
        assert_eq!(markdown, Some("## Header 2".to_string()));
    }

    #[test]
    fn test_translate_h3() {
        let markdown = translate_fragment(r#"<h3>Header 3</h3>"#, "h3");
        assert_eq!(markdown, Some("### Header 3".to_string()));
    }

    #[test]
    fn test_translate_h4() {
        let markdown = translate_fragment(r#"<h4>Header 4</h4>"#, "h4");
        assert_eq!(markdown, Some("#### Header 4".to_string()));
    }

    #[test]
    fn test_translate_div() {
        let markdown =
            translate_fragment(r#"<div><p>Some text</p><p>And more text</p></div>"#, "div");
        assert_eq!(markdown, Some("Some text\n\nAnd more text".to_string()));
    }

    #[test]
    fn test_filename_from_url_one_fragment() {
        let url = Url::parse("https://www.fake_site.org/one_fragment").unwrap();
        assert_eq!(filename_from_url(&url), "one_fragment");
    }

    #[test]
    fn test_filename_from_url_two_fragments() {
        let url = Url::parse("https://www.fake_site.org/one_fragment/two_fragment").unwrap();
        assert_eq!(filename_from_url(&url), "one_fragment_two_fragment");
    }

    #[test]
    fn test_filename_from_url_with_extension() {
        let url = Url::parse("https://www.fake_site.org/one_fragment.png").unwrap();
        assert_eq!(filename_from_url(&url), "one_fragment.png");
    }
}
