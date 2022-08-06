use warp::http::Uri;
use scraper::{Html, Selector};
use serde::de::Error;

// Stores Open Graph protocol metadata
struct OpenGraph {
    url: Uri,
    title: String,
    // url and title are mandatory metadata
    site_name: Option<String>, // (url fallbacks on scraped uri, title fallbacks on url)
    description: Option<String>,
    image: Option<Uri>,
    video: Option<Uri>,
}

struct Website(Uri);
pub async fn getDocument(url: Uri) -> Result<String, reqwest::Error>{
    let resp = Html::parse_document(
        &reqwest::get("https://cinnamondev.github.io/")
        .await?
        .text()
        .await?
    );
    let selector = Selector::parse("meta").unwrap();
    let a = resp.select(&selector).map(|e| {e.html()}).collect();
    Ok(a)
}