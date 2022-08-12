use crate::OpenGraph::OgType::Website;

#[derive(Debug, Default, Clone)]
pub struct OgMeta<'a> {
    // "Required" types.
    title: &'a str,
    url: &'a str,
    ogType: OgType, // unmarked types are treated as Website.
    image: Vec<OgImage>,

    // These 2 are "optional" but are configurable with defaults.
    determiner: &'a str,
    locale: &'a str,

    opt: OptionalMeta<'a>,
    quirks: bool,
}
#[derive(Debug, Default, Clone)]
pub struct OptionalMeta<'a> {
    // "Optional" types.
    audio: Option<&'a str>,
    description: Option<&'a str>,
    alt_locale: Vec<&'a str>,
    site_name: Option<&'a str>,
    video: Option<&'a str>,
}

enum MetaFactoryError {
    MissingParams(Vec<MissingError>)
}

enum MissingError {
    Title,
    Url,
    Type,
    Image,
}
pub struct MetaBuilder<'a> {
    // "Required" types.
    title: &'a str,
    url: &'a str,
    ogType: OgType,
    image: Vec<OgImage>,
    locale: &'a str,
    determiner: &'a str,

    opt: OptionalMeta<'a>,
}
impl<'a> MetaBuilder<'a> {
    pub fn new(url: &'a str) -> MetaBuilder {
        MetaBuilder{
            title: "",
            url,
            ogType: OgType::default(),
            image: vec![],
            locale: "",
            determiner: "",
            opt: Default::default()
        }.with_url(url)
    }
    pub fn with_url(self, url: &'a str) -> Self {
        MetaBuilder {
            url,
            ..self
        }
    }
    fn or_fallback_title(self) -> Self {
        MetaBuilder {
            title: if self.title.is_empty() {
                self.url
            } else {
                self.title
            },
            ..self
        }
    }
    pub fn with_title(self, title: &'a str) -> Self {
        MetaBuilder {
            title,
            ..self
        }
    }
    pub fn with_type(self, ogType: OgType) -> Self {
        MetaBuilder {
            ogType,
            ..self
        }
    }
    pub fn with_determiner(self, det: &'a str) -> Self {
        MetaBuilder {
            determiner: match det {
                "auto" => match self.title.to_lowercase().chars().nth(0) {
                    Some('a'|'e'|'i'|'o'|'u') => "an",
                    _ => "a",   // Sensible default, error or not a vowel.
                    },
                _ => det
            },
            ..self
        }
    }
    pub fn with_locale(self, locale: &'a str) -> Self {
        MetaBuilder {
            locale,
            ..self
        }
    }
    fn or_fallback_locale(self) -> Self {
        MetaBuilder {
            locale: if self.locale.is_empty() {
                "en_US"
            } else {
                self.locale
            },
            ..self
        }
    }
    pub fn with_audio(self, url: &'a str) -> Self {
        MetaBuilder {
            opt: OptionalMeta {
                audio: Some(url),
                ..self.opt
            },
            ..self
        }
    }
    pub fn with_description(self, desc: &'a str) -> Self {
        MetaBuilder {
            opt: OptionalMeta {
                description: Some(desc),
                ..self.opt
            },
            ..self
        }
    }
    pub fn build(self, allow_quirks: bool) -> Result<OgMeta<'a>, ()> {
        let tmp = self.or_fallback_locale();
        if allow_quirks {
            // Allow for "quirks" in the current implementation.
            // If enabled, "required" tags will be provided fallbacks.
            let full = tmp.or_fallback_title();
            Ok(OgMeta {
                title: full.title,
                url: full.url,
                ogType: full.ogType,
                image: full.image,
                determiner: full.determiner,
                locale: full.locale,
                opt: full.opt,

                quirks: true,
            })
        } else {
            match tmp {
                mb if mb.title.is_empty() => Err(()),
                _ => Ok(OgMeta {
                        title: tmp.title,
                        url: tmp.url,
                        ogType: tmp.ogType,
                        image: tmp.image,
                        determiner: tmp.determiner,
                        locale: tmp.locale,
                        opt: tmp.opt,

                        quirks: false,
                    }
                )
            }
        }
    }
}
/// The type of your object, e.g., "video.movie". Depending on the type you specify, other properties may also be required.
///
/// Contains the Object Type.
#[derive(Debug, Default, Clone)]
pub enum OgType {
    #[default]
    Website,
    Article(OgArticle),
    Book(OgBook),
    Profile(OgProfile),
    Video(//OgVideo
    ),
    Music(//OgMusic
    ),
}

/// Describes an image fully. Not all values have to be used, only url.
#[derive(Debug, Default, Clone)]
pub struct OgImage {
    url: String,
    secure_url: Option<String>,
    mimeType: Option<String>,
    width: Option<u16>,
    height: Option<u16>,
    alt: Option<String>,
}


// No vertical objects

/// Describes a user profile. Can be a og:type or stored by another
#[derive(Debug, Default, Clone)]
pub struct OgProfile {
    first_name: String,
    last_name: String,
    username: String,
    gender: String, // the protocol says this is an enum but i disagree
}
/// Describes a book. Used as og:type
#[derive(Debug, Default, Clone)]
pub struct OgBook {
    author: Vec<String>,
    isbn: String,
    rel_time: String,
    tag: Vec<String>,
}
/// Describes an article. Used as og:type
#[derive(Debug, Default, Clone)]
pub struct OgArticle {  // TODO: fix types.
pub_time: String,
    mod_time: String,
    exp_time: String,
    author: Vec<String>,
    section: String,
    tag: Vec<String>,
}

// Vertical objects

// TODO: Music object.
// seems annoying, music.song
// contains many albums and
// music.album contains song

// TODO: Video object. seems too annoying rn.
/*
/// Describes a generic "video". Used in many instances of OgVideo.
struct VideoData {
    actor: vec<OgProfile>,
    role: String,
    director: Vec<OgProfile>,
    writer: Vec<OgProfile>,
    duration: u16,
    rel_date: String,
    tag: Vec<String>,
}

/// Describes a OpenGraph video type. Contains information about a video.
pub enum OgVideo {
    Movie(VideoData),
    TvShow(VideoData),
    Episode {
        ep: VideoData,
        series: OgVideo::TV_Show
    },
    Other(VideoData),
}
 */
