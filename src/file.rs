use image::io::Reader as ImageReader;
use serde::Serialize;
use std::error::Error;
use std::io::Cursor;
use std::path::PathBuf;

pub struct ImageDownloader;

#[derive(Serialize)]
pub struct ImageDownloadResult {
    pub name: String,
    pub path: PathBuf,
    pub saved: bool,
    pub error: Option<String>,
}

impl ImageDownloader {
    /// Downloads a file from a given url and saves it to disk
    pub async fn download_file(
        url: &str,
        dir: &PathBuf,
    ) -> Result<ImageDownloadResult, Box<dyn Error>> {
        let fragments = url.split("https://i.redd.it/").collect::<Vec<&str>>();
        let name = fragments[1];

        let path = dir.join(name);

        let bytes = reqwest::get(url).await?.bytes().await?;

        let img = ImageReader::new(Cursor::new(bytes))
            .with_guessed_format()?
            .decode()?;

        let file = match img.save(&path) {
            Ok(_) => {
                println!(
                    "Successfully downloaded image {:?} (saved to {:?})",
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
                println!("Failed to download image {:?}", name);

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
