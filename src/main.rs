use reqwest;
use std::result::Result;
type Value = serde_json::Map<std::string::String, serde_json::Value>;
type Error = Box<dyn std::error::Error>;

fn main() {

    get_list();
}

#[tokio::main]
async fn get_list() -> Result<Value, Error> {

    let client = reqwest::Client::new();
    let url = "https://gist.githubusercontent.com/0eta0/fc2130c0e0c756712e085e0287ec5908/raw/a176b9cd340b18e74cf5eef66f1b5116bcc5d627/images.json";
    let body = client.get(url)
        .send()
        .await?
        .text()
        .await?;

    let json: serde_json::Value = serde_json::from_str(&body)?;
    let obj = json.as_object().unwrap();

    for (key,value) in obj.iter() {
        println!("{}\t{}",key,value);
    }

    Ok(obj)
}