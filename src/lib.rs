use std::collections::HashMap;
use std::error::Error;
use url::Url;
extern crate reqwest;
use base64::{decode, encode};
use serde::Deserialize;

#[derive(Debug, Default)]
pub struct Gist {
    user: String,
    repo: String,
    refer: String,
    path: String,
    start: i32,
    end: i32,
}

/// Github file content response
#[derive(Deserialize)]
pub struct ContentResponse {
    name: String,
    content: String,
}

pub fn get_content(link: &str) -> Result<ContentResponse, Box<dyn Error>> {
    let res: ContentResponse = reqwest::get(link)?.json()?;
    println!("{:?}", res.content);
    Ok(res)
}

/// 根据github文件地址生成gist
/// includes: line range
pub fn generate(link: &str) -> &str {
    return "gist url";
}

/// 46d5465b35188ddf03bd884220b1bb5934461bf2
/// Example url: https://github.com/AliasT/public/blob/master/lib/react.cjs.js#L14-L16
///                                | user | repo |   refer   |        path     |
///
impl Gist {
    pub fn parse(link: &str) -> Result<Option<Gist>, Box<dyn Error>> {
        let file_url_object = Url::parse(link)?;
        let path = file_url_object.path();
        let re =
            regex::Regex::new(r"/(?P<user>.+?)/(?P<repo>.+?)/blob/(?P<refer>.+?)/(?P<file>.+)$")?;
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

    pub fn create(&self) {}
}

#[test]
fn test_parse() {
    if let Some(gist) =
        Gist::parse("https://github.com/AliasT/public/blob/master/lib/react.cjs.js#L14-L16")
            .unwrap()
    {
        assert!(gist.user == "AliasT");
        assert!(gist.repo == "public");
        assert!(gist.refer == "master");
        assert!(gist.path == "lib/react.cjs.js");
    }
}

#[test]
fn test_get_content() {
    let res =
        get_content("https://api.github.com/repos/aliast/online-ide-discovery/contents/README.md")
            .unwrap();
    assert!(res.name == "README.md")
}
