use std::sync::{Arc, Mutex};

use demo::{handle, Item, Request};
use tracing::span::Attributes;
use tracing::Subscriber;
use tracing_subscriber::layer::{Context, Layer};
use tracing_subscriber::prelude::*;

#[derive(Clone, Default)]
struct Capture {
    names: Arc<Mutex<Vec<String>>>,
}

impl Capture {
    fn names(&self) -> Vec<String> {
        self.names.lock().unwrap().clone()
    }
}

impl<S: Subscriber> Layer<S> for Capture {
    fn on_new_span(&self, attrs: &Attributes<'_>, _id: &tracing::span::Id, _ctx: Context<'_, S>) {
        self.names
            .lock()
            .unwrap()
            .push(attrs.metadata().name().to_string());
    }
}

fn pedido() -> Request {
    Request {
        customer_id: 1,
        items: vec![
            Item { unit_price_cents: 1000, quantity: 2 },
            Item { unit_price_cents: 500, quantity: 1 },
        ],
    }
}

#[tokio::test(flavor = "current_thread")]
async fn baseline_emite_span_externo_e_spans_de_io_sem_spans_de_cpu() {
    let capture = Capture::default();
    let subscriber = tracing_subscriber::registry().with(capture.clone());
    let _guard = tracing::subscriber::set_default(subscriber);

    handle(pedido()).await.unwrap();

    let names = capture.names();
    for esperado in ["handle", "load_customer", "persist"] {
        assert!(
            names.iter().any(|name| name == esperado),
            "span {esperado} ausente; encontrados: {names:?}"
        );
    }
    for proibido in ["validate", "price", "apply_rules", "render"] {
        assert!(
            !names.iter().any(|name| name == proibido),
            "span de CPU {proibido} não deveria existir; encontrados: {names:?}"
        );
    }
}
