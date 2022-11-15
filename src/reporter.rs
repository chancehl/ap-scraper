use crate::ap_image::ArtPornSubredditImage;
use chrono::Utc;
use serde::Serialize;
use std::error::Error;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;

#[derive(Serialize)]
pub struct Report {
    processed_images: Vec<ArtPornSubredditImage>,
}

impl Report {
    pub fn new(processed_images: Vec<ArtPornSubredditImage>) -> Self {
        Self { processed_images }
    }

    pub fn write_to_disk(self, dir: &PathBuf) -> Result<PathBuf, Box<dyn Error>> {
        let time = Utc::now();
        let file_location = dir.join(format!("report-{0}.json", time.format("%d/%m/%Y %H:%M")));
        let file = File::create(dir.join(&file_location))?;

        let mut writer = BufWriter::new(file);

        serde_json::to_writer(&mut writer, &self.processed_images)?;
        writer.flush()?;

        Ok(file_location)
    }
}
