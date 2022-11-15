use constants::{REDDIT_CDN_ASSET_REGEX, REDDIT_URL, SUBREDDIT_URL};
use file::{ImageDownloadResult, ImageDownloader};
use regex::Regex;
use reporter::Report;
use scraper::Selector;
use std::error::Error;

mod constants;
mod file;
mod reporter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let list_page_html = get_html(SUBREDDIT_URL).await?;
    let list_page_document = scraper::Html::parse_document(&list_page_html);

    let post_container_selector = Selector::parse("[data-testid=\"post-container\"]").unwrap();
    let a_selector = Selector::parse("a").unwrap();

    let posts = list_page_document.select(&post_container_selector);

    let mut results: Vec<ImageDownloadResult> = Vec::new();

    for post in posts {
        let a = post.select(&a_selector).next().unwrap();
        let href = a.value().attr("href").unwrap_or("No href found");

        let post_detail_url = format!("{0}{1}", REDDIT_URL, href);
        let post_detail_html = get_html(&post_detail_url).await?;
        let post_detail_document = scraper::Html::parse_document(&post_detail_html);

        let post_container = post_detail_document
            .select(&post_container_selector)
            .next()
            .unwrap();

        let url = post_container
            .select(&a_selector)
            .next()
            .unwrap()
            .value()
            .attr("href")
            .unwrap();

        if Regex::new(REDDIT_CDN_ASSET_REGEX).unwrap().is_match(url) {
            let result = ImageDownloader::download_file(&url, "./imgs").await?;

            results.push(result);
        } else {
            println!(
                "Skipping img {:?} because it does not pass the Reddit CDN regex check",
                url
            );
        }
    }

    // report
    match Report::new(results).write_to_disk("./imgs") {
        Ok(loc) => println!("Success! Saved report to {0}", loc),
        Err(err) => panic!(
            "Could not write write report to disk. Error: \n\n {:?}",
            err
        ),
    }

    Ok(())
}

async fn get_html(url: &str) -> Result<String, Box<dyn Error>> {
    let resp = reqwest::get(url).await?.text().await?;

    return Ok(resp);
}
