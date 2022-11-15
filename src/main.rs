use file::ImageDownloadResult;
use image::io::Reader as ImageReader;
use regex::Regex;
use reporter::Report;
use scraper::Selector;
use std::error::Error;
use std::io::Cursor;

mod file;
mod reporter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let list_page_html = get_html("https://reddit.com/r/artporn").await?;
    let list_page_document = scraper::Html::parse_document(&list_page_html);

    let post_container_selector = Selector::parse("[data-testid=\"post-container\"]").unwrap();
    let a_selector = Selector::parse("a").unwrap();

    let posts = list_page_document.select(&post_container_selector);

    let mut results: Vec<ImageDownloadResult> = Vec::new();

    for post in posts {
        let a = post.select(&a_selector).next().unwrap();
        let href = a.value().attr("href").unwrap_or("No href found");

        let post_detail_url = format!("https://reddit.com{0}", href);
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

        if Regex::new(r"^https://i.redd.it/").unwrap().is_match(url) {
            let result = download_file(&url).await?;

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
        Ok(_) => println!("Success! Saved report to ./imgs."),
        Err(e) => panic!("Could not write write report to disk. Error: \n\n {:?}", e),
    }

    Ok(())
}

async fn get_html(url: &str) -> Result<String, Box<dyn Error>> {
    let resp = reqwest::get(url).await?.text().await?;

    return Ok(resp);
}

async fn download_file(url: &str) -> Result<ImageDownloadResult, Box<dyn Error>> {
    let fragments = url.split("https://i.redd.it/").collect::<Vec<&str>>();
    let name = fragments[1];

    let path = format!("./imgs/{0}", name);

    let bytes = reqwest::get(url).await?.bytes().await?;

    let img = ImageReader::new(Cursor::new(bytes))
        .with_guessed_format()?
        .decode()?;

    let file = match img.save(&path) {
        Ok(_) => {
            println!("Successfully downloaded image {0} (saved to ./imgs)", name);

            ImageDownloadResult {
                name: name.to_string(),
                path: path,
                saved: true,
                error: None,
            }
        }
        Err(e) => {
            println!("Failed to download image {0}", name);

            ImageDownloadResult {
                name: name.to_string(),
                path: path,
                saved: false,
                error: Some(e.to_string()),
            }
        }
    };

    Ok(file)
}
