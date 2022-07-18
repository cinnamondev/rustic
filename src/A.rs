impl FromStr for Website {
    type Err = ScrapeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<Uri>() {
            Ok(u) => {
                if let None = u.host() {
                    Err(ScrapeError::NoHost)
                } else {
                    Ok(Website {
                        url:u
                    })
                }
            },
            Err(e) => Err(ScrapeError::BadUri(e.to_string())),
        }
    }
}