mod card;
use reqwest::blocking::*;

fn main() {
    println!("Hello, world!");
    let client = Client::new();
    let url = "https://db.ygoprodeck.com/api/v7/cardinfo.php?";
    let resp = client.get(url)
        .query(&[("name", "Endymion, the Mighty Master of Magic")])
        .send()
        .unwrap()
        .json::<card::response_card::YGOProResponse>()
        .unwrap();
    println!("{}", serde_json::to_string_pretty(&resp).unwrap());
}
