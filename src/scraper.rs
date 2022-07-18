use warp::http::Uri;
// Stores Open Graph protocol metadata
struct OpenGraph {
    site_name: String,
    title: String,
    description: String,
    image: Uri,
    url: Uri,
    video: Uri,
}

struct Website(Uri);
struct Scraper;
impl Scraper {

}