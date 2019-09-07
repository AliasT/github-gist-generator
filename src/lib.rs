use std::collections::HashMap;
use std::error::Error;
use std::str;

extern crate reqwest;

use base64::{decode, encode};
use reqwest::header::{HeaderMap, HeaderValue};
use serde::Deserialize;
use url::Url;

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
// #[derive(Deserialize)]
// pub struct ContentResponse {
//     name: String,
//     content: String,
// }

/// Get lines in a range
pub fn get_content(link: &str) -> Result<String, Box<dyn Error>> {
    let client = reqwest::Client::new();
    let res: String = client
        .get(link)
        // 发送accept请求头，获取原数据
        .header("Accept", "application/vnd.github.raw")
        .send()?
        .text()?;

    // fake
    let start = 10;
    let end = 15;

    // 跳过start前的行
    let mut lines = res.lines().skip(start - 1);
    let mut ret: Vec<String> = Vec::new();

    for n in start..=end {
        let line = lines.next().unwrap();
        ret.push(String::from(line));
    }

    // 重新拼接\n
    Ok(ret.join("\n"))
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
            regex::Regex::new(r"/(?P<user>.+?)/(?P<repo>.+?)/blob/(?P<refer>.+?)/(?P<file>.+)#L(?P<start>\d+)-L(?P<end>\d+)$")?;
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
        assert!(gist.start == 14);
        assert!(gist.end == 16);
    }
}

#[test]
fn test_get_content() {
    let res =
        get_content("https://api.github.com/repos/aliast/online-ide-discovery/contents/README.md")
            .unwrap();
    println!("{}", res)
}
