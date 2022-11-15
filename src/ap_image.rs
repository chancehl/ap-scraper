use serde::Serialize;

use crate::file::ImageDownloadResult;

#[derive(Serialize)]
pub struct ArtPornSubredditImageMetadata {
    pub title: String,
}

#[derive(Serialize)]
pub struct ArtPornSubredditImage {
    pub id: String,
    pub download_results: ImageDownloadResult,
    pub metadata: ArtPornSubredditImageMetadata,
}
