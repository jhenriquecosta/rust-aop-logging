use std::sync::{Arc, Mutex};

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

#[aop_macros::aop_span(name = "etapa_renomeada")]
fn etapa_renomeada(n: u32) -> u32 {
    n + 1
}

#[test]
fn aop_span_com_name_renomeia_o_span() {
    let capture = Capture::default();
    let subscriber = tracing_subscriber::registry().with(capture.clone());
    let _guard = tracing::subscriber::set_default(subscriber);

    assert_eq!(etapa_renomeada(1), 2);

    let names = capture.names();
    assert!(
        names.iter().any(|name| name == "etapa_renomeada"),
        "span renomeado ausente: {names:?}"
    );
    assert!(
        !names.iter().any(|name| name == "etapa_renomeada_fn"),
        "nome antigo não deveria existir: {names:?}"
    );
}

#[aop_macros::aop_span]
async fn soma_assincrona(a: u32, b: u32) -> u32 {
    a + b
}

#[tokio::test(flavor = "current_thread")]
async fn aop_span_em_async_preserva_retorno_e_emite_span() {
    let capture = Capture::default();
    let subscriber = tracing_subscriber::registry().with(capture.clone());
    let _guard = tracing::subscriber::set_default(subscriber);

    assert_eq!(soma_assincrona(2, 3).await, 5);

    let names = capture.names();
    assert!(
        names.iter().any(|name| name == "soma_assincrona"),
        "span da async fn ausente: {names:?}"
    );
}

#[aop_macros::aop_span]
fn falha_sincrona() -> Result<(), String> {
    Err("falhou".to_string())
}

#[aop_macros::aop_span]
async fn falha_assincrona() -> Result<(), String> {
    Err("falhou".to_string())
}

#[tokio::test(flavor = "current_thread")]
async fn aop_span_preserva_erro_em_sync_e_async() {
    assert_eq!(falha_sincrona(), Err("falhou".to_string()));
    assert_eq!(falha_assincrona().await, Err("falhou".to_string()));
}
