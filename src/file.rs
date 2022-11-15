use image::io::Reader as ImageReader;
use serde::Serialize;
use std::error::Error;
use std::io::Cursor;

pub struct ImageDownloader;

#[derive(Serialize)]
pub struct ImageDownloadResult {
    pub name: String,
    pub path: String,
    pub saved: bool,
    pub error: Option<String>,
}

impl ImageDownloader {
    /// Downloads a file from a given url and saves it to disk
    pub async fn download_file(
        url: &str,
        dir: &str,
    ) -> Result<ImageDownloadResult, Box<dyn Error>> {
        let fragments = url.split("https://i.redd.it/").collect::<Vec<&str>>();
        let name = fragments[1];

        let path = format!("{0}/{1}", dir, name);

        let bytes = reqwest::get(url).await?.bytes().await?;

        let img = ImageReader::new(Cursor::new(bytes))
            .with_guessed_format()?
            .decode()?;

        let file = match img.save(&path) {
            Ok(_) => {
                println!(
                    "Successfully downloaded image {0} (saved to {1})",
                    name, path
                );

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
}
