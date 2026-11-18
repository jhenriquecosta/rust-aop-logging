# Fatia 4 — README e comparação

## O que construir

README do projeto com:

- o que é AOP em Rust (macros de atributo) e por que blocos CPU-bound não são instrumentados automaticamente;
- o pipeline e os dois modos, com trechos reais da saída `baseline` vs `aop`;
- como a macro `#[aop_span]` é aplicada e o que ela faz (em prosa, sem prometer o que não foi validado);
- comparação com `aspect-rs` (0.1.x, sem releases, sem integração com tracing/OpenTelemetry — ver research 0001);
- caminho de produção para OTLP, deixando claro que o demo não o implementa;
- instruções para rodar (`cargo run -- baseline`, `cargo run -- aop`, `cargo test`) e a versão de Rust necessária.

## Critérios de aceite

- [ ] O README explica AOP em Rust, mostra o antes/depois e compara com aspect-rs.
- [ ] O README descreve o caminho OTLP de produção e o que fica fora do demo.
- [ ] As instruções de execução funcionam no workspace.

## Bloqueada por

Fatias 1, 2 e 3.
