# TDD Progress Log

## Demo de AOP em Rust — spans por etapa — task 0001
Started: 2026-11-18
Stack: Rust 1.100 · `cargo test` · tracing 0.1.44 · tracing-subscriber 0.3.23 · tokio 1.54.1

### Behaviors planned
- [x] 1. `handle` calcula o total do pedido (caminho feliz) ✓ 2026-11-18
- [x] 2. baseline emite o span externo e os spans de IO, sem spans de CPU ✓ 2026-11-18
- [x] 3. binário `demo baseline` imprime o trace e sai com 0 ✓ 2026-11-18
- [x] 4. erro do cliente de IO propaga como `Err` ✓ 2026-11-18

### Cycle log
| # | Behavior | Result | Notes |
|---|----------|--------|-------|
| 1 | `handle` calcula o total do pedido | ✓ GREEN | implementação mínima, ainda sem spans |
| 2 | baseline emite span externo e spans de IO, sem spans de CPU | ✓ GREEN | `#[instrument]` no `handle`, `load_customer` e `persist` |
| 3 | binário `demo baseline` imprime o trace e sai com 0 | ✓ GREEN | `FmtSpan::CLOSE`; campos `time.busy`/`time.idle` confirmados |
| 4 | erro do cliente de IO propaga como `Err` | ✓ GREEN | id 404 simula `CustomerNotFound`; refactor: doc comments da API pública |

### Open issues found during implementation
- nenhuma até aqui
