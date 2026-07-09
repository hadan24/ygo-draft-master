mod card;
use reqwest::blocking::*;

fn main() {
    println!("Hello, world!");
    let client = Client::new();
    let url = "https://db.ygoprodeck.com/api/v7/cardinfo.php?";
    let resp = client.get(url)
        .query(&[("format", "tcg")])
        .send()
        .unwrap()
        .json::<card::response_card::YGOProResponse>()
        .unwrap();

    // Jul 9 2026 -> 13903 cards (including Skills), ~14.5 mb
    println!("{}, {}", resp.len(), size_of_val(&resp));

    // to view process memory usage to get above num
    let _keep_term_open = std::io::Read::read(&mut std::io::stdin(), &mut []);
}
