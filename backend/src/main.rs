use ygo_draft_backend::card;
use axum::routing::get;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let tcp_listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await.expect("Should be able to set up listener on hard-coded localhost");
    let app = axum::Router::new()
        .route("/", get(|| async { "Hallo :D 🦀" }))
        .route("/lens", get(lens));

    axum::serve(tcp_listener, app).await
        .expect("Should never return/error (https://docs.rs/axum/latest/axum/serve/fn.serve.html)");
}


async fn lens() -> String {
    let client = reqwest::Client::new();
    let resp = client.get("https://db.ygoprodeck.com/api/v7/cardinfo.php?")
        .query(&[("format", "tcg")])
        .send().await
        .unwrap()
        .json::<card::response_card::YGOProResponse>().await
        .unwrap()
        .data;

    // filter to only "tcg Skill cards", id is Maliss special case
    let skills: Vec<card::response_card::ResponseCard> = resp.iter()
        .filter_map(|c| {
            if c.race == card::response_card::Race::Other && c.id != 20726052 { Some(c.clone()) }
            else { None }
        })
        .collect();

    // Jul 9 2026 -> 13903 cards (including Skills), ~15.1 mb
    format!("{}, {}\n{:?}", resp.len(), skills.len(), axum::Json(skills))
}