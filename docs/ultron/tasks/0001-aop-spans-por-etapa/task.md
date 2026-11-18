# Task 0001-aop-spans-por-etapa — Demonstração de AOP em Rust: spans por etapa de processamento

tipo: feature
branch: feature/0001-aop-spans-por-etapa
publicacao: pendente
estado: concluído
worktree: .claude/worktrees/feature-0001-aop-spans-por-etapa
responsavel: jhenriquecosta@hotmail.com
next_skill: 

## Objetivo

Criar um projeto de demonstração do uso de AOP (programação orientada a aspectos) em Rust para instrumentação de logging/telemetria, motivado por um problema real de latência: em produção, vários blocos de processamento aparecem unidos em um único span, e não é possível saber quanto tempo cada passo levou. Como esses blocos não fazem IO, não são instrumentados automaticamente.

## O que a demonstração precisa mostrar

- Separação de spans por etapa de processamento, inclusive para blocos CPU-bound sem IO.
- Duração por passo registrada e visível (tracing/OpenTelemetry).
- A parte AOP: a instrumentação aplicada de forma declarativa (ex.: attribute macro procedural, #[instrument] ou macro própria), sem espalhar código de telemetria pela lógica de negócio.
- Comparação entre o que é instrumentado automaticamente (IO/middleware) e o que exige instrumentação explícita.

## Contexto de produção (motivador)

Blocos unidos em um único span; sem IO, não há instrumentação automática; o objetivo é medir o tempo de cada passo do processamento.

## Fora de escopo (a confirmar na descoberta)

- Aplicar a mudança no sistema de produção (outro repositório).
- Trocar a stack de observabilidade do time.

## Decisões da descoberta (2026-11-18)

- **Formato:** serviço async simulado (tokio), sem servidor HTTP; o binário executa o pipeline como se fosse uma requisição.
- **AOP:** `#[instrument]` do tracing como aspecto pronto + macro de atributo própria (`#[aop_span]`, crate proc-macro) para os passos CPU-bound.
- **Pipeline:** `handle` (span externo) → `validate` (CPU) → `load_customer` (IO simulado) → `price` (CPU) → `apply_rules` (CPU) → `persist` (IO simulado) → `render` (CPU).
- **Modos:** `baseline` (span externo e spans de IO automáticos; passos CPU sem span, reproduzindo a produção) e `aop` (span por etapa).
- **Saída:** `tracing-subscriber` fmt com `FmtSpan::CLOSE`; OTLP documentado como caminho de produção, não implementado.
- **Estrutura:** workspace Cargo — `aop-macros` (proc-macro) + `demo` (binário).
- **aspect-rs:** apenas comparação no README; nenhuma dependência. É 0.1.x, sem releases, e não integra tracing/OpenTelemetry.
- **Evidência:** testes com camada de captura provam que cada etapa emite span próprio no modo `aop` e que o `baseline` mantém um único span.

## Critérios de aceitação

- [x] O workspace compila e roda com `cargo run -- baseline` e `cargo run -- aop`.
- [x] No modo `aop`, cada etapa do pipeline emite um span próprio com duração visível na saída.
- [x] No modo `baseline`, aparecem o span externo e os spans de IO, e nenhum span de etapa CPU (reproduz o problema).
- [x] A instrumentação das etapas CPU vem da macro `#[aop_span]`, não de chamadas manuais de span espalhadas.
- [x] O README explica o que é AOP em Rust, mostra o antes/depois e compara com `aspect-rs` e OTLP.
- [x] Os testes de captura verificam os spans emitidos em ambos os modos.

## Questões abertas (para /ultron-research)

- Versões e APIs atuais de `tracing`, `tracing-subscriber` (`FmtSpan::CLOSE`), `tokio` e das crates de proc-macro (`syn`, `quote`, `proc-macro2`); edição e MSRV.
- Padrão correto para attribute macro em `async fn` (spans via `Instrument`, preservando o `Future`).
- Precisão/limitações do timing de spans do `tracing` para blocos CPU-bound (tempo busy/idle só do future atual).
- Fatos do `aspect-rs` a registrar como evidência (crates 0.1.x, ausência de integração com tracing, maturidade).

## Fatias publicadas

| # | Fatia | Issue | Estado |
|---|---|---|---|
| 1 | Pipeline executável no modo `baseline` | — (local) | concluída · verificada 2026-11-18 |
| 2 | Macro `#[aop_span]` e modo `aop` | — (local) | concluída · verificada 2026-11-18 |
| 3 | Macro em `async fn` + captura de spans | — (local) | concluída · verificada 2026-11-18 |
| 4 | README e comparação | — (local) | concluída · verificada 2026-11-18 |

Os corpos estão em `slices/` (fatias 01–04), na ordem de execução.

## Exceções registradas

- Portão `logging` (médio) do coder desviado em 2026-11-18: o PRD fixa a configuração do subscriber (`fmt` + `FmtSpan::CLOSE`) e a estrutura de spans por etapa; uma convenção separada duplicaria a decisão. Aprovado pelo Operador.
- Portão `verify` do fechamento (2026-11-18): o `verify_all.sh` do arsenal reconhece dotnet, maven, node e python — não Rust — e sai `2` (nada verificado). A verificação foi executada manualmente com `cargo build`, `cargo clippy --workspace --all-targets -- -D warnings` e `cargo test` (9 testes verdes).
