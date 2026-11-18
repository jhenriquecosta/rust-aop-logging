use demo::{handle, handle_aop, Item, Request};
use tracing_subscriber::fmt::format::FmtSpan;

#[tokio::main]
async fn main() {
    let mode = std::env::args().nth(1);

    if !matches!(mode.as_deref(), Some("baseline") | Some("aop")) {
        eprintln!("uso: demo baseline|aop");
        std::process::exit(2);
    }

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

    let resultado = match mode.as_deref() {
        Some("baseline") => handle(req).await,
        Some("aop") => handle_aop(req).await,
        _ => unreachable!(),
    };

    match resultado {
        Ok(resp) => println!("resposta: {resp:?}"),
        Err(err) => {
            eprintln!("erro: {err}");
            std::process::exit(1);
        }
    }
}
