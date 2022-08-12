
use std::io;
use std::str::FromStr;
use html5ever::tendril::*;
use html5ever::tokenizer::{BufferQueue, Tag};
use html5ever::tokenizer::{StartTag, TagToken};
use html5ever::tokenizer::{Token, TokenSink, TokenSinkResult, Tokenizer, TokenizerOpts,};
use html5ever::interface::QualName;
use html5ever::{ns, namespace_url, local_name, LocalName, Attribute};
//use crate::OpenGraph::{OgMeta};

struct Document {
    uri: String,
    //contents: Html,
}

#[derive(Default)]
pub struct MetaPrinter {
    property: String,
    content: String,
}

impl MetaPrinter {
    fn with_property(self, prop: String) -> Self {
        MetaPrinter {
            property: prop,
            ..self
        }
    }
    fn with_content(self, cont: String) -> Self {
        MetaPrinter {
            content: cont,
            ..self
        }
    }
}

impl TokenSink for MetaPrinter {
    type Handle = ();

    fn process_token(&mut self, token: Token, line_number: u64) -> TokenSinkResult<Self::Handle> {
        match token {
            TagToken(Tag {kind: StartTag, name: local_name!("meta"), attrs, .. }) => {
                let iter = attrs.into_iter();
                let _properties = iter.clone()
                    .filter(|a| a.name.local == local_name!("property"))
                    .collect::<Vec<Attribute>>();
                let _contents = iter
                    .filter(|a| a.name.local == local_name!("content"))
                    .collect::<Vec<Attribute>>();

                match (_properties.first(), _contents.first()) {
                    (Some(p), Some(c)) => {
                        dbg!("{} {}", p,c);
                        TokenSinkResult::Continue
                    },
                    _ => TokenSinkResult::Continue
                }
            },
            _ => TokenSinkResult::Continue
        }
    }
}
