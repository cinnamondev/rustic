mod WebDocument;

use std::str::FromStr;
use html5ever::tendril::StrTendril;
use html5ever::tokenizer::{BufferQueue, Tokenizer, TokenizerOpts};
use crate::WebDocument::MetaPrinter;

//use crate::OpenGraph::MetaBuilder;
//mod scraper;
//use serde::{Deserialize,Serialize};
//use warp::Filter;
//use warp::http::{Uri,uri::{InvalidUri}};

fn main() {
    //let a = MetaBuilder::new("www.google.com").build(false);
    //dbg!("{}",a);
    let sink = MetaPrinter::default();
    let doc = r#"
    <html id="a" lang="en"><!--If no JS default to light.--><head>
    <meta charset="UTF-8">
    <meta http-equiv="X-UA-Compatible" content="IE=edge">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <link rel="stylesheet" href="css/index.css">
    <script src="js/theme.js"></script>
    <!--<script src="js/hover.js"></script>-->
    <script src="js/index.js"></script>

    <title>Cinnamondev - Projects</title>
    <meta property="og:title" content="cindev">
<meta property="og:type" content="profile:cinnamondev">
<meta property="og:url" content="https://cinnamondev.github.io">
<meta property="og:image" content="https://ia.media-imdb.com/images/rock.jpg">
</head><body>
        </body></html>
    "#;
    let mut buf = BufferQueue::new();
    buf.push_back(StrTendril::from_str(doc).unwrap());

    let mut tok = Tokenizer::new(
        sink,
        TokenizerOpts::default(),
    );
    let _ = tok.feed(&mut buf);
    tok.end();
}
