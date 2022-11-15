use crate::file::ImageDownloadResult;
use chrono::Utc;
use serde::Serialize;
use std::error::Error;
use std::fs::File;
use std::io::{BufWriter, Write};

#[derive(Serialize)]
pub struct Report {
    results: Vec<ImageDownloadResult>,
}

impl Report {
    pub fn new(results: Vec<ImageDownloadResult>) -> Self {
        Self { results }
    }

    pub fn write_to_disk(self, dir_path: &str) -> Result<String, Box<dyn Error>> {
        let time = Utc::now();
        let file_location = format!("{0}/report-{1}.json", dir_path, time);
        let file = File::create(&file_location)?;

        let mut writer = BufWriter::new(file);

        serde_json::to_writer(&mut writer, &self.results)?;
        writer.flush()?;

        Ok(file_location)
    }
}
