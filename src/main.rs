use ap_image::{ArtPornSubredditImage, ArtPornSubredditImageMetadata};
use clap::Parser;
use constants::{REDDIT_CDN_ASSET_REGEX, REDDIT_URL, SUBREDDIT_URL};
use file::ImageDownloader;
use regex::Regex;
use reporter::Report;
use scraper::Selector;
use std::{error::Error, path};

mod ap_image;
mod constants;
mod file;
mod reporter;

/// A simple program for scraping https://reddit.com/r/artporn for high-resolution art.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// The loation to save the file
    #[clap(short, long, default_value = "./imgs")]
    outdir: path::PathBuf,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let outdir = args.outdir;

    let list_page_html = get_html(SUBREDDIT_URL).await?;
    let list_page_document = scraper::Html::parse_document(&list_page_html);

    let post_container_selector = Selector::parse("[data-testid=\"post-container\"]").unwrap();
    let a_selector = Selector::parse("a").unwrap();
    let post_title_selector = Selector::parse("h1").unwrap();

    let posts = list_page_document.select(&post_container_selector);

    let mut processed_images: Vec<ArtPornSubredditImage> = Vec::new();

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

        let post_title = post_container
            .select(&post_title_selector)
            .next()
            .unwrap()
            .inner_html();

        if Regex::new(REDDIT_CDN_ASSET_REGEX).unwrap().is_match(url) {
            let result = ImageDownloader::download_file(&url, &outdir).await?;

            let processed_image = ArtPornSubredditImage {
                id: url.to_string(),
                download_results: result,
                metadata: ArtPornSubredditImageMetadata { title: post_title },
            };

            processed_images.push(processed_image);
        } else {
            println!(
                "Skipping img {:?} because it does not pass the Reddit CDN regex check",
                url
            );
        }
    }

    // report
    match Report::new(processed_images).write_to_disk(&outdir) {
        Ok(loc) => println!("Success! Saved report to {:?}", loc),
        Err(err) => panic!("Could not write write report to disk (error: {:?}).", err),
    }

    Ok(())
}

async fn get_html(url: &str) -> Result<String, Box<dyn Error>> {
    let resp = reqwest::get(url).await?.text().await?;

    return Ok(resp);
}
