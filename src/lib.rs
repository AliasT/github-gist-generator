use std::borrow::Cow;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::str;

extern crate dotenv;
extern crate reqwest;

use serde::{Deserialize, Serialize};

#[derive(Debug, Default)]
pub struct Gist<'a> {
    user: &'a str,
    repo: &'a str,
    refer: &'a str,
    path: &'a str,
    start: i32,
    end: i32,
}

#[derive(Debug, Serialize)]
pub struct File<'a> {
    content: &'a str,
}

#[derive(Debug, Serialize)]
pub struct Payload<'a> {
    description: &'a str,
    files: HashMap<&'a str, File<'a>>,
    public: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GistResponse<'a> {
    html_url: Cow<'a, str>,
}

/// Get lines in a range
pub fn get_content(link: &str, start: usize, end: usize) -> Result<String, Box<dyn Error>> {
    let client = reqwest::Client::new();
    let res: String = client
        .get(link)
        // 发送accept请求头，获取原数据
        .header("Accept", "application/vnd.github.raw")
        .send()?
        .text()?;

    // 跳过start前的行
    let mut lines = res.lines().skip(start - 1);
    let mut ret: Vec<&str> = Vec::new();

    // @todo: may be better way to select lines
    for _ in start - 1..=end - 1 {
        let line = lines.next().unwrap();
        ret.push(line);
    }

    // join lines
    Ok(ret.join("\n"))
}

/// Parse a github repo url to get file meta
///
/// Example url: https://github.com/AliasT/public/blob/master/lib/react.cjs.js#L14-L16
///                                | user | repo |   refer   |        path     |
impl<'a, 'b> Gist<'a> {
    pub fn parse(path: &str) -> Result<Gist, Box<dyn Error>> {
        // currently must specify line range
        let re =
            regex::Regex::new(r"/(?P<user>.+?)/(?P<repo>.+?)/blob/(?P<refer>.+?)/(?P<file>.+?)#L(?P<start>\d+)-L(?P<end>\d+)$")?;

        let gist = match re.captures(path) {
            Some(caps) => {
                let start: i32 = match caps.name("start") {
                    Some(value) => value.as_str().parse().unwrap(),
                    None => 0,
                };

                let end: i32 = match caps.name("end") {
                    Some(value) => value.as_str().parse().unwrap(),
                    None => 0,
                };

                Gist {
                    user: caps.name("user").unwrap().as_str(),
                    repo: caps.name("repo").unwrap().as_str(),
                    refer: caps.name("refer").unwrap().as_str(),
                    path: caps.name("file").unwrap().as_str(),
                    start,
                    end,
                }
            }
            None => panic!("no matches"),
        };
        Ok(gist)
    }

    pub fn create(&self) -> Result<Cow<str>, Box<dyn Error>> {
        // dotenv parse
        dotenv::dotenv().ok();
        let client = reqwest::Client::new();
        // /repos/aliast/online-ide-discovery/contents/README.md
        let file_url = format!(
            "https://api.github.com/repos/{}/{}/contents/{}?ref={}",
            self.user, self.repo, self.path, self.refer
        );
        let content =
            get_content(file_url.as_str(), self.start as usize, self.end as usize).unwrap();
        // use github token to create gists
        let mut map = HashMap::new();
        map.insert(
            "gists.js",
            File {
                content: content.as_str(),
            },
        );
        let payload = Payload {
            public: true,
            description: "",
            files: map,
        };
        let username = env::var("USERNAME").unwrap();
        let token = env::var("TOKEN").unwrap();
        let res: GistResponse = client
            .post("https://api.github.com/gists")
            .basic_auth(username, Some(token))
            .json(&payload)
            .send()?
            .json()?;

        Ok(res.html_url)
    }
}

#[test]
fn test_create() {
    let gist = Gist::parse("/AliasT/public/blob/master/lib/react.cjs.js#L14-L16").unwrap();
    gist.create().unwrap();
}

#[test]
fn test_parse() {
    let gist = Gist::parse("/AliasT/public/blob/master/lib/react.cjs.js#L14-L25").unwrap();
    assert!(gist.user == "AliasT");
    assert!(gist.repo == "public");
    assert!(gist.refer == "master");
    assert!(gist.path == "lib/react.cjs.js");
}
