fn main() {
    println!("Hello, world!");
}

struct Gist {
    user: String,
    repo: String,
    refer: String,
    path: String,
    start: i32,
    end: i32,
}

/// 根据github文件地址生成gist
/// includes: line range
fn generate(url: &str) -> &str {
    return "gist url";
}

/// Example url: https://github.com/AliasT/public/blob/master/lib/react.cjs.js#L14-L16
///                                |:user | :repo |    :ref  |      :path     |
fn parse(url: &str) -> Option<Gist> {
    return None;
}
