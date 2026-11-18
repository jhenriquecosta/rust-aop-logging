use demo::{handle, Item, Request};
use tracing_subscriber::fmt::format::FmtSpan;

#[tokio::main]
async fn main() {
    match std::env::args().nth(1).as_deref() {
        Some("baseline") => {
            tracing_subscriber::fmt()
                .with_span_events(FmtSpan::CLOSE)
                .with_max_level(tracing::Level::INFO)
                .init();

            let req = Request {
                customer_id: 1,
                items: vec![
                    Item { unit_price_cents: 1000, quantity: 2 },
                    Item { unit_price_cents: 500, quantity: 1 },
                ],
            };

            match handle(req).await {
                Ok(resp) => println!("resposta: {resp:?}"),
                Err(err) => {
                    eprintln!("erro: {err}");
                    std::process::exit(1);
                }
            }
        }
        _ => {
            eprintln!("uso: demo baseline");
            std::process::exit(2);
        }
    }
}
