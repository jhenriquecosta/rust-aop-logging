# Fatia 1 — Pipeline executável no modo baseline

## O que construir

Workspace Cargo (edição 2021, `rust-version = "1.85"`) com a crate `demo`: um binário tokio que executa o pipeline simulado de ponta a ponta e imprime o trace no terminal.

- Pipeline: `handle` (span externo) → `validate` (CPU) → `load_customer` (IO) → `price` (CPU) → `apply_rules` (CPU) → `persist` (IO) → `render` (CPU).
- Os corpos CPU são funções puras (`validate_impl`, `price_impl`, `apply_rules_impl`, `render_impl`), sem span.
- Os passos de IO são clientes falsos (`CustomerClient`, `OrderRepository`) com `tokio::time::sleep` e `#[instrument]` — representam a instrumentação automática de produção.
- CLI: `cargo run -- baseline` executa o pipeline uma vez sob o span `handle`.
- Subscriber: `tracing_subscriber::fmt` com `with_span_events(FmtSpan::CLOSE)` e timestamps habilitados.
- Dependências fixadas: `tracing 0.1.44`, `tracing-subscriber 0.3.23`, `tokio 1.54.1` (features `macros`, `rt-multi-thread`, `time`).

## Critérios de aceite

- [ ] `cargo run -- baseline` imprime o span `handle` e os spans de IO (`load_customer`, `persist`).
- [ ] Nenhum span de etapa CPU (`validate`, `price`, `apply_rules`, `render`) aparece nesse modo.
- [ ] A saída mostra a duração dos spans (tempo ocupado/ocioso) com timestamps habilitados.
- [ ] O workspace compila com as versões fixadas, edição 2021 e `rust-version = "1.85"`.

## Bloqueada por

Nenhuma — pode começar imediatamente. Pré-requisito de ambiente: toolchain Rust instalada (`rustc`/`cargo`/`rustup` ausentes no host).
