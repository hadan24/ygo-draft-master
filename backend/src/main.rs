use ygo_draft_backend::card;
use axum::routing::get;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let tcp_listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await.expect("Should be able to set up listener on hard-coded localhost");
    let app = axum::Router::new()
        .route("/", get(|| async { "Hallo :D 🦀" }))
        .route("/time_db", get(time_db));

    axum::serve(tcp_listener, app).await
        .expect("Should never return/error (https://docs.rs/axum/latest/axum/serve/fn.serve.html)");
}


async fn time_db() -> String {
    let client = reqwest::Client::new();
    let start = std::time::Instant::now();
    let ygopro_cards = client.get("https://db.ygoprodeck.com/api/v7/cardinfo.php?")
        .query(&[("format", "tcg")])
        .send()
        .await.unwrap()
        .json::<card::response_card::YGOProResponse>()
        .await.unwrap()
        .data
        .into_iter()
        .filter_map(|rcard| {
            if (rcard.race == card::response_card::Race::Other || rcard.card_type.contains("Token")) && rcard.id != 20726052 {
                return None;
            }
            let id = rcard.id;
            match card::ygo_card::YGOCard::new_from_response(rcard) {
                Ok(card) => Some(Ok(card)),
                Err(e) => Some(Err(format!("failed to create from {id}: {e}")))
            }
        })
        .collect::< Result<Vec<card::ygo_card::YGOCard>, String> >()
        .unwrap();
    let elapsed = start.elapsed();
    format!("Took : {elapsed:?} to convert {} cards", ygopro_cards.len())
}