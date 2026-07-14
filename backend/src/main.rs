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

    // Jul 9 2026 -> 13903 cards (including Skills), ~15.1 mb
    println!("{}", resp.len());

    let resp: Vec<&card::response_card::ResponseCard> = resp.iter()
        .filter(|c| c.race == card::response_card::Race::Other && c.id != 20726052)
        .collect();
    println!("{}", resp.len());
    println!("{resp:?}");

    // to view process memory usage to get above num
    let _keep_term_open = std::io::Read::read(&mut std::io::stdin(), &mut []);
}
