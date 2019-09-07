use std::collections::HashMap;
use std::error::Error;
use std::str;

extern crate reqwest;

use serde::Serialize;

#[derive(Debug, Default)]
pub struct Gist {
    user: String,
    repo: String,
    refer: String,
    path: String,
    start: i32,
    end: i32,
}

#[derive(Debug, Serialize)]
pub struct File {
    content: String,
}

#[derive(Debug, Serialize)]
pub struct Payload {
    description: String,
    files: HashMap<String, File>,
    public: bool,
}

/// Get lines in a range
pub fn get_content(link: &str, start: usize, mut end: usize) -> Result<String, Box<dyn Error>> {
    let client = reqwest::Client::new();
    let res: String = client
        .get(link)
        // 发送accept请求头，获取原数据
        .header("Accept", "application/vnd.github.raw")
        .send()?
        .text()?;

    // 跳过start前的行
    let mut lines = res.lines().skip(start - 1);
    let mut ret: Vec<String> = Vec::new();

    // @todo: may be better way to select lines
    for _ in start - 1..=end - 1 {
        let line = lines.next().unwrap();
        ret.push(String::from(line));
    }

    // join lines
    Ok(ret.join("\n"))
}

/// Parse a github repo url to get file meta
///
/// Example url: https://github.com/AliasT/public/blob/master/lib/react.cjs.js#L14-L16
///                                | user | repo |   refer   |        path     |
impl Gist {
    pub fn parse(path: &str) -> Result<Option<Gist>, Box<dyn Error>> {
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

                Some(Gist {
                    user: String::from(caps.name("user").unwrap().as_str()),
                    repo: String::from(caps.name("repo").unwrap().as_str()),
                    refer: String::from(caps.name("refer").unwrap().as_str()),
                    path: String::from(caps.name("file").unwrap().as_str()),
                    start,
                    end,
                })
            }
            // @todo:
            None => None,
        };
        Ok(gist)
    }

    pub fn create(&self) -> Result<(), Box<dyn Error>> {
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
        map.insert(String::from("gists.js"), File { content });
        let payload = Payload {
            public: true,
            description: String::from(""),
            files: map,
        };

        let res = client
            .post("https://api.github.com/gists")
            .header(
                "Authorization",
                "token 9b113ea3b2a00caa8016d0dceeb75dd1f777cb7a",
            )
            .json(&payload)
            .send()?;
        println!("{:?}", res);
        Ok(())
    }
}

#[test]
fn test_create() {
    if let Some(gist) = Gist::parse("/AliasT/public/blob/master/lib/react.cjs.js#L14-L16").unwrap()
    {
        gist.create().unwrap();
    }
}

#[test]
fn test_parse() {
    if let Some(gist) = Gist::parse("/AliasT/public/blob/master/lib/react.cjs.js#L14-L16").unwrap()
    {
        assert!(gist.user == "AliasT");
        assert!(gist.repo == "public");
        assert!(gist.refer == "master");
        assert!(gist.path == "lib/react.cjs.js");
    }
}
