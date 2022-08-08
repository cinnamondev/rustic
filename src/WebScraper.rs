#![feature(once_cell)]

use warp::http::Uri;
use scraper::{Html, Selector, selector};
use cssparser::ParseError;
use serde::de::Error;
use std::lazy::Lazy;
use warp::test::request;

enum ScraperError {
    BadRequest(reqwest::Error)
}

enum GrabberError {
    BadSelector
}

#[derive(Default)]
struct MetaData {
    title: String,
    wtype: String,
    image: String,
    url: String,
    audio: Option<String>,
    description: Option<String>,
    determiner: Option<String>,
    locale: Option<String>,
    alt_locale: Option<String>,
    site_name: Option<String>,
    video: Option<String>,
}
struct Website {
    url: Uri,
    contents: Option<Html>
}

impl Website {
    async fn get_document(uri: Uri) -> Result<Html, reqwest::Error> {
        let response = Html::parse_document(
            reqwest::get::<Uri>(&*uri)
            .await?
            .text()
            .await?
        );
        Ok(response)
    }
    async fn new(uri: Uri) -> Self {
        Self {
            url: &*uri,
            contents:
                match Website::get_document(uri).await {
                    Ok(t) => Some(t),
                    Err(e) => None
                }
        }
    }

}

impl OpenGraph {
    fn new(uri: Uri) -> Self {
        Self {
            url: uri.to_string(),
        }
    } 
}

pub async fn scrape_meta(url: Uri) -> Result<OpenGraph, dyn Error> {
    

    todo!()
}

pub async fn meta_from_document(url: Uri) -> Result<String, reqwest::Error>{
    let resp = Html::parse_document(
        &reqwest::get("https://cinnamondev.github.io/")
        .await?
        .text()
        .await?
    );
    let selector = Selector::parse(r#"meta[property^="og"]"#).unwrap();
    let a = resp.select(&selector).map(|e| {e.html()}).collect();
    Ok(a)
}

fn 

impl TryFrom<Website> for OpenGraph {
    type Error = ParserError;
    fn try_from(value: Html) -> Result<Self, Self::Error> {
       let string = value.text();
       let s = Selector::parse(r#"meta[property^="og"]"#);
       match s {
           Ok(selector) => {
               let meta = OpenGraph::
               let a = value.select(&selector). ;
           },
           Err(_) => {
               // wasn't sure how to handle this crate... :/ TODO: fix later?
               Err(GrabberError::BadSelector)
           }
       }
    }
}
