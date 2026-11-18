# TDD Progress Log

## Demo de AOP em Rust — spans por etapa — task 0001
Started: 2026-11-18
Stack: Rust 1.100 · `cargo test` · tracing 0.1.44 · tracing-subscriber 0.3.23 · tokio 1.54.1

### Behaviors planned
- [x] 1. `handle` calcula o total do pedido (caminho feliz) ✓ 2026-11-18
- [x] 2. baseline emite o span externo e os spans de IO, sem spans de CPU ✓ 2026-11-18
- [x] 3. binário `demo baseline` imprime o trace e sai com 0 ✓ 2026-11-18
- [x] 4. erro do cliente de IO propaga como `Err` ✓ 2026-11-18
- [x] 5. modo `aop` emite um span por etapa (fatia 2) ✓ 2026-11-18
- [x] 6. `#[aop_span(name = "...")]` renomeia o span (fatia 2) ✓ 2026-11-18
- [x] 7. macro em `async fn` preserva retorno e erro (fatia 3) ✓ 2026-11-18
- [x] 8. as etapas do modo `aop` ficam aninhadas sob `handle` (fatia 3) ✓ 2026-11-18

### Cycle log
| # | Behavior | Result | Notes |
|---|----------|--------|-------|
| 1 | `handle` calcula o total do pedido | ✓ GREEN | implementação mínima, ainda sem spans |
| 2 | baseline emite span externo e spans de IO, sem spans de CPU | ✓ GREEN | `#[instrument]` no `handle`, `load_customer` e `persist` |
| 3 | binário `demo baseline` imprime o trace e sai com 0 | ✓ GREEN | `FmtSpan::CLOSE`; campos `time.busy`/`time.idle` confirmados |
| 4 | erro do cliente de IO propaga como `Err` | ✓ GREEN | id 404 simula `CustomerNotFound`; refactor: doc comments da API pública |
| 5 | modo `aop` emite um span por etapa | ✓ GREEN | crate `aop-macros`; wrappers `validate`/`price`/`apply_rules`/`render` |
| 6 | `#[aop_span(name = "...")]` renomeia o span | ✓ GREEN | teste de captura; `name` já suportado pela macro |
| 7 | macro em `async fn` preserva retorno e erro | ✓ GREEN | future instrumentado com `tracing::Instrument` |
| 8 | etapas aninhadas sob `handle` no modo `aop` | ✓ GREEN | camada de captura agora registra o pai de cada span |

### Open issues found during implementation
- nenhuma até aqui

## Log rotativo por data na demo de AOP — task 0002 · PRD-0002
Started: 2026-11-18
Stack: Rust 1.100 · `cargo test` · tracing 0.1.44 · tracing-subscriber 0.3.23 · tracing-appender 0.2.5 · tokio 1.54.1

### Behaviors planned
- [x] 1. com `DEMO_LOG_DIR`, o modo `baseline` cria o arquivo do dia ✓ 2026-11-18
- [x] 2. o arquivo não contém ANSI ✓ 2026-11-18
- [x] 3. o arquivo do `baseline` tem os spans de IO e não os de CPU ✓ 2026-11-18
- [x] 4. o arquivo do `aop` tem as seis etapas sob `handle` ✓ 2026-11-18
- [x] 5. o `stdout` continua com o trace nos dois modos ✓ 2026-11-18
- [x] 6. sem `DEMO_LOG_DIR`, o destino é `logs/` sob o cwd ✓ 2026-11-18

### Cycle log
| # | Behavior | Result | Notes |
|---|----------|--------|-------|
| 1 | baseline com `DEMO_LOG_DIR` cria o arquivo do dia | ✓ GREEN | `tracing-appender 0.2.5`; dois layers; guard no `main`; flush antes do `exit(1)` |
| 2 | o arquivo não contém ANSI | ✓ GREEN | `with_ansi(false)`; o RED expôs o cache de campos entre layers — o layer do arquivo vai primeiro (terminal perde a cor só nos nomes de campo) |
| 3 | conteúdo do baseline no arquivo | ✓ GREEN | regressão travada após o tracer bullet |
| 4 | conteúdo do aop no arquivo | ✓ GREEN | hierarquia `handle:<etapa>` afirmada |
| 5 | stdout nos dois modos | ✓ GREEN | idem; `time.busy` presente |
| 6 | destino padrão `logs/` sem env | ✓ GREEN | fallback relativo ao cwd; `.gitignore` com `/logs/` |

### Open issues found during implementation
- `docs/ultron/agents/stack.md` continua dizendo `stack: n/d`, embora o repositório seja Rust — fora do escopo desta fatia; reportar ao final.

