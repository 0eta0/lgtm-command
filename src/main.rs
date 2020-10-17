use reqwest;

use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;

use std::env;
use std::result::Result;
use rand::Rng;

type ObjectMap = serde_json::Map<std::string::String, serde_json::Value>;
type Error = Box<dyn std::error::Error>;

fn main() {

    let links = match get_list() {
        Ok(links) => links,
        Err(err) => panic!(err.to_string())
    };
    let args: Vec<String> = env::args().collect();

    if args.iter().count() < 2 {
        println!("\n===== PARAMs =====\n\nlist        => return index of image\nget         => return markdown styled image url randomly\nget number  => return markdown styled image url\n");
        return;
    };

    if "list" == args[1] {
        cmd_list(links);
        return;
    };

    if "get" == args[1] {
        let mut idx: u32 = 0;
        if args.iter().count() < 3 {
            idx = match args[2].parse::<u32>() {
                Ok(val) => val,
                Err(_) => 0
            };
        };
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

    let idxs: Vec<String> = indexes(links);
    let rnd: i32 = rand::thread_rng().gen_range(1, idxs.iter().count() as i32);
    let idx: usize = match idx {
        0 => (rnd - 1) as usize,
        _ => (idx - 1) as usize
    };
    if let Some(url) = idxs.get(idx) {
        clip(url);
        return;
    }
    println!("Invalid number! No available image found!");
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
