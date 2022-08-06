use std::str::FromStr;
//mod scraper;
use serde::{Deserialize,Serialize};
use warp::Filter;
use warp::http::{Uri,uri::{InvalidUri}};

mod WebScraper;
#[tokio::main]
async fn main() -> Result<(), reqwest::Error>{

    //println!("aaa {:#?}",resp);
    let a =WebScraper::getDocument(Uri::from_str("www.google.com").unwrap()).await?;
    println!("{}",a);

    return Ok(());
    //WebScraper::getDocument(Uri::from_str(&"www.google.com").unwrap());
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let scrape = warp::get()
        .and(warp::path("scrape"))
        .and(warp::query()
            .map(|v:Website| {
                match v.url.parse::<Uri>() {
                    Ok(w) => warp::reply::json(&[1,2,3,4]),
                    Err(e) => warp::reply::json(&(e.to_string()))
                }
            }));


    warp::serve(scrape)
        .run(([127, 0, 0, 1], 7878))
        .await;
}

#[derive(Deserialize)]
struct Website {
    url: String,
}

#[derive(Serialize)]
enum ScrapeError {
    BadUri(String),
    NotAbsolute(String),
    BadSchema(String),
    NoHost
}
