use std::default;
use std::error::Error;
use url::{ParseError, Url};

#[derive(Debug, Default)]
pub struct Gist {
    user: String,
    repo: String,
    refer: String,
    path: String,
    start: i32,
    end: i32,
}

/// 根据github文件地址生成gist
/// includes: line range
pub fn generate(link: &str) -> &str {
    return "gist url";
}

/// Example url: https://github.com/AliasT/public/blob/master/lib/react.cjs.js#L14-L16
///                                | user | repo |   refer   |        path     |
///
impl Gist {
    pub fn parse(link: &str) -> Result<Option<Gist>, Box<dyn Error>> {
        let file_url_object = Url::parse(link)?;
        let path = file_url_object.path();
        let re = regex::Regex::new(r"/(?P<user>.+?)/(?P<repo>.+?)/blob/(?P<refer>.+?)/(?P<file>.+)$")?;
        let gist = match re.captures(path) {
            Some(caps) => Some(Gist {
                user: String::from(caps.name("user").unwrap().as_str()),
                repo: String::from(caps.name("repo").unwrap().as_str()),
                refer: String::from(caps.name("refer").unwrap().as_str()),
                path: String::from(caps.name("file").unwrap().as_str()),
                start: 0,
                end: 0,
            }),
            // @todo:
            None => None,
        };

        Ok(gist)
    }

    pub create(&self) {

    }
}

#[test]
fn test_parse() {
    if let Some(gist) =
        Gist::parse("https://github.com/AliasT/public/blob/master/lib/react.cjs.js#L14-L16").unwrap()
    {
        assert!(gist.user == "AliasT");
        assert!(gist.repo == "public");
        assert!(gist.refer == "master");
        assert!(gist.path == "lib/react.cjs.js");
    }
}
