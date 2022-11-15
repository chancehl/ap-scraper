use serde::Serialize;

#[derive(Serialize)]
pub struct ImageDownloadResult {
    pub name: String,
    pub path: String,
    pub saved: bool,
    pub error: Option<String>,
}
