use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};

use demo::{handle, handle_aop, Item, Request};
use tracing::span::Attributes;
use tracing::Subscriber;
use tracing_subscriber::layer::{Context, Layer};
use tracing_subscriber::prelude::*;

type Spans = Vec<(String, Option<String>)>;
type SpanIds = HashMap<u64, String>;

#[derive(Clone, Default)]
struct Capture {
    spans: Arc<Mutex<Spans>>,
    ids: Arc<Mutex<SpanIds>>,
    entraram: Arc<Mutex<HashSet<String>>>,
    fecharam: Arc<Mutex<HashSet<String>>>,
}

impl Capture {
    fn names(&self) -> Vec<String> {
        self.spans
            .lock()
            .unwrap()
            .iter()
            .map(|(name, _)| name.clone())
            .collect()
    }

    fn parent_of(&self, name: &str) -> Option<String> {
        self.spans
            .lock()
            .unwrap()
            .iter()
            .find(|(span, _)| span == name)
            .and_then(|(_, parent)| parent.clone())
    }

    fn entrou(&self, name: &str) -> bool {
        self.entraram.lock().unwrap().contains(name)
    }

    fn fechou(&self, name: &str) -> bool {
        self.fecharam.lock().unwrap().contains(name)
    }
}

impl<S: Subscriber> Layer<S> for Capture {
    fn on_new_span(&self, attrs: &Attributes<'_>, id: &tracing::span::Id, ctx: Context<'_, S>) {
        let name = attrs.metadata().name().to_string();
        let parent = ctx
            .current_span()
            .id()
            .and_then(|parent_id| self.ids.lock().unwrap().get(&parent_id.into_u64()).cloned());
        self.ids.lock().unwrap().insert(id.into_u64(), name.clone());
        self.spans.lock().unwrap().push((name, parent));
    }

    fn on_enter(&self, id: &tracing::span::Id, _ctx: Context<'_, S>) {
        if let Some(name) = self.ids.lock().unwrap().get(&id.into_u64()).cloned() {
            self.entraram.lock().unwrap().insert(name);
        }
    }

    fn on_close(&self, id: tracing::span::Id, _ctx: Context<'_, S>) {
        if let Some(name) = self.ids.lock().unwrap().get(&id.into_u64()).cloned() {
            self.fecharam.lock().unwrap().insert(name);
        }
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

#[tokio::test(flavor = "current_thread")]
async fn aop_aninha_as_etapas_sob_o_span_handle() {
    let capture = Capture::default();
    let subscriber = tracing_subscriber::registry().with(capture.clone());
    let _guard = tracing::subscriber::set_default(subscriber);

    handle_aop(pedido()).await.unwrap();

    let names = capture.names();
    for etapa in [
        "validate",
        "load_customer",
        "price",
        "apply_rules",
        "persist",
        "render",
    ] {
        assert!(
            names.iter().any(|name| name == etapa),
            "span {etapa} ausente; encontrados: {names:?}"
        );
        assert_eq!(
            capture.parent_of(etapa).as_deref(),
            Some("handle"),
            "span {etapa} deveria estar sob `handle`; encontrados: {names:?}"
        );
        assert!(
            capture.entrou(etapa),
            "span {etapa} não foi entered; encontrados: {names:?}"
        );
        assert!(
            capture.fechou(etapa),
            "span {etapa} não foi fechado; encontrados: {names:?}"
        );
    }
}
