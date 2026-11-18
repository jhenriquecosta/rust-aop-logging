# Fatia 3 — Macro em async fn + captura de spans

## O que construir

- Suporte de `#[aop_span]` a `async fn`: o future é instrumentado com `tracing::Instrument`, com o span entered a cada poll.
- Camada de captura de spans (uma `Layer` própria) usada nos testes — os eventos sintetizados por `FmtSpan::CLOSE` não são eventos `tracing` reais.
- Testes:
  - modo `aop` emite o conjunto de spans por etapa com a hierarquia correta sob `handle`;
  - modo `baseline` não emite spans de etapa CPU;
  - a macro preserva o valor de retorno em `fn` e em `async fn`;
  - a macro preserva a propagação de erro (`Result`/`Err`) em `fn` e em `async fn`.

## Critérios de aceite

- [ ] `cargo test` passa e cobre presença/ausência dos spans nos dois modos.
- [ ] Há teste de `#[aop_span]` em `fn` síncrona e em `async fn`, incluindo retorno e erro.
- [ ] A suíte não depende de coletor externo nem dos eventos sintetizados pelo fmt.

## Bloqueada por

Fatia 2 (macro e modo aop).
