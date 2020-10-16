use std::env;
use reqwest;
use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;
use std::result::Result;
type ObjectMap = serde_json::Map<std::string::String, serde_json::Value>;
type Error = Box<dyn std::error::Error>;

fn main() {

    let links = match get_list() {
        Ok(links) => links,
        Err(err) => panic!(err.to_string())
    };
    let args: Vec<String> = env::args().collect();

    if args.iter().count() < 2 {
        println!("\n===== PARAMs =====\n\nlist        => return index of image\nget number  => return markdown styled image url\n");
        return;
    };

    if "list" == args[1] {
        cmd_list(links);
        return;
    };

    if "get" == args[1] {
        let idx: u32 = args[2].parse().unwrap();
        cmd_get(idx, links);
        return;
    };
}

#[tokio::main]
async fn get_list() -> Result<ObjectMap, Error> {

    let client = reqwest::Client::new();
    let url = "https://gist.githubusercontent.com/0eta0/fc2130c0e0c756712e085e0287ec5908/raw/a176b9cd340b18e74cf5eef66f1b5116bcc5d627/images.json";
    let body = client.get(url)
        .send()
        .await?
        .text()
        .await?;

    let links: ObjectMap = serde_json::from_str(&body)?;
    Ok(links)
}

fn cmd_list(links: ObjectMap) {

    for (index, map) in links.iter().enumerate() {
        println!("{} => {}", index + 1, map.0);
    }
}

fn cmd_get(idx: u32, links: ObjectMap) {

    if let Some(url) = indexes(links).get(idx as usize) {
        clip(url);
    }
}

fn indexes(links: ObjectMap) -> Vec<String> {

    let mut vec = Vec::new();
    for (_, v) in links.iter() {
        vec.push(v.to_string());
    }
    vec
}

fn clip(link: &String) {

    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    let markdown = format!("![]({})", link[1..link.chars().count() - 1].to_owned());
    ctx.set_contents(markdown).unwrap();
}
