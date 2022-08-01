use warp::http::Uri;
use scraper::Html;
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
fn getDocument(url: Uri) -> Html {
    
    todo!()
}