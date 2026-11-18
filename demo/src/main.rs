use demo::{handle, handle_aop, Item, Request};
use tracing_subscriber::fmt;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

/// Instala os dois destinos do trace: o arquivo diário e o stdout.
/// O guard devolvido precisa viver até o fim do `main` para o buffer esvaziar.
fn instalar_tracing(dir: std::path::PathBuf) -> tracing_appender::non_blocking::WorkerGuard {
    std::fs::create_dir_all(&dir).expect("criar o diretório de log");
    let appender = tracing_appender::rolling::RollingFileAppender::builder()
        .rotation(tracing_appender::rolling::Rotation::DAILY)
        .filename_prefix("demo.log")
        .max_log_files(7)
        .build(&dir)
        .expect("criar o appender do log");
    let (non_blocking, guard) = tracing_appender::non_blocking(appender);

    tracing_subscriber::registry()
        .with(tracing_subscriber::filter::LevelFilter::INFO)
        // O layer do arquivo vem primeiro: o cache de campos formatados é
        // compartilhado por tipo de formatter, e a primeira layer a gravar
        // os campos define o modo ANSI deles. Com o arquivo primeiro, o
        // arquivo fica limpo (o terminal ganha os campos sem cor, e mantém
        // cor no timestamp, nível e nome do span).
        .with(
            fmt::Layer::new()
                .with_writer(non_blocking)
                .with_ansi(false)
                .with_span_events(FmtSpan::CLOSE),
        )
        .with(
            fmt::Layer::new()
                .with_writer(std::io::stdout)
                .with_span_events(FmtSpan::CLOSE),
        )
        .init();

    guard
}

#[tokio::main]
async fn main() {
    let mode = std::env::args().nth(1);

    if !matches!(mode.as_deref(), Some("baseline") | Some("aop")) {
        eprintln!("uso: demo baseline|aop");
        std::process::exit(2);
    }

    let dir = std::env::var_os("DEMO_LOG_DIR")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| std::path::PathBuf::from("logs"));
    let guard = instalar_tracing(dir);

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
            // `process::exit` não roda o drop do guard; soltá-lo aqui
            // garante o flush antes de sair.
            drop(guard);
            std::process::exit(1);
        }
    }
}
