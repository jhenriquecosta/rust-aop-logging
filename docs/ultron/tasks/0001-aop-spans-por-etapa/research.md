# Research 0001 — Fundamentos para o demo de AOP em Rust

**Data:** 2026-11-18
**Contexto:** task 0001-aop-spans-por-etapa (feature), antes do PRD
**Context7:** indisponível — o MCP deste ambiente exige autenticação OAuth; doc de bibliotecas obtida da fonte oficial (docs.rs), como manda a degradação graciosa
**Tavily:** indisponível (`TAVILY_API_KEY` ausente) — pesquisa web via `web_search`/fetch

## Perguntas investigadas

### 1. Versões atuais das crates e MSRV

**Resposta:** as versões publicadas em 2026-11-18 (API do crates.io) e seus requisitos:

| Crate | Versão | Publicada | MSRV | Edição |
|---|---|---|---|---|
| `tracing` | 0.1.44 | 2025-12-18 | 1.65 | 2018 |
| `tracing-attributes` | 0.1.31 | 2025-11-26 | 1.65 | — |
| `tracing-subscriber` | 0.3.23 | 2026-03-13 | 1.65 | 2018 |
| `tokio` | 1.54.1 | 2026-10-21 | 1.85 | 2021 |
| `syn` | 3.0.10 | 2026-11-09 | 1.71 | 2021 |
| `quote` | 1.0.47 | 2026-07-19 | — | — |
| `proc-macro2` | 1.0.107 | 2026-07-19 | — | — |

O `syn` está na linha 3.x (3.0.0 em 2026-07-18; última 2.x, 2.0.119, em 2026-07-15). O 3.0 traz mudanças quebradas em relação ao 2.x (structs `*Modifiers`, `Type::BareFn` → `Type::FnPtr`, mudanças em `Receiver`, `Punctuated::pop`, entre outras), então exemplos da internet escritos para syn 2 podem não compilar tal e qual.

**Implicação:** o workspace pode usar edição 2021 e `rust-version = "1.85"` (o maior MSRV entre as dependências, ditado pelo tokio). O `syn 3` exige a feature `full` para `ItemFn`.

**Fontes:** API do crates.io (primária, consultada em 2026-11-18); docs.rs/syn 3.0.10 (09/11/2026); GitHub dtolnay/syn releases (3.0.0, breaking changes).
**Confiança:** alta para versões e MSRV. Para "syn 3 é estável": alta por evidência indireta (docs.rs "latest", release normal sem pré-release, tag assinada), mas nenhuma página usa literalmente a palavra "stable".

### 2. `FmtSpan::CLOSE` mostra a duração de cada span?

**Resposta:** sim, mas de um jeito específico. `FmtSpan::CLOSE` sintetiza **um evento quando o span é fechado** (drop). Com timestamps habilitados, esse evento carrega o **busy time** (tempo total em que o span esteve *entered*) e o **idle time** (tempo em que o span existiu mas não estava *entered*). Não existe um campo separado de "duração total" — busy + idle é o tempo de vida do span.

- `FmtSpan::NONE` é o padrão: sem `with_span_events`, nenhuma duração aparece.
- `FmtSpan` exige as features `fmt` e `std`, ambas default; a feature `time` **não** é necessária para busy/idle (ela habilita `UtcTime`/`LocalTime`).
- `without_time()` remove os timestamps e, com eles, os campos busy/idle.
- Os eventos sintetizados pelo fmt **não são eventos `tracing` reais**: só aparecem no output desse formatter e não chegam a outras `Layer`s.

**Implicação para o demo:** o binário usa `with_span_events(FmtSpan::CLOSE)` para mostrar a duração por etapa no terminal. Já os **testes não podem** depender desses eventos sintetizados — precisam de uma `Layer` própria para registrar a criação/entrada/fechamento dos spans.

**Fontes:** docs.rs/tracing-subscriber 0.3.23 — `fmt/format/struct.FmtSpan.html`, `fmt/struct.SubscriberBuilder.html` (with_span_events), `fmt/index.html`, `fmt/format/struct.Full.html`.
**Confiança:** alta. Ressalva: os nomes literais dos campos (`time.busy`/`time.idle`) não aparecem nos docs lidos; os docs os descrevem como "busy time" e "idle time". O demo confirma o formato em execução.

### 3. `#[instrument]` em funções async — o que ele faz e o que limita

**Resposta:** `#[instrument]` é uma **macro procedural de atributo** (da crate `tracing-attributes`, reexportada por `tracing` com a feature `attributes`). Ela cria e entra um span **a cada chamada** da função: nível `INFO` por padrão, nome da função e todos os argumentos como campos (via `Value` ou `fmt::Debug`).

Opções documentadas: `skip(...)`/`skip_all`, `fields(...)` (inclusive aninhados e `%` para `Display`), `name`, `target`, `level`, `ret` (evento no retorno; `ret(Display)`) e `err` (evento no `Err`). Funciona em funções síncronas, genéricas, métodos e `async fn`; `const fn` não é suportada.

Sobre async: a página do atributo diz apenas que `async fn`s "may also be instrumented". O mecanismo documentado está no trait **`Instrument`**: uma futura instrumentada tem o span **entered a cada poll e no drop**. As páginas não afirmam explicitamente que `#[instrument]` usa `Instrument` internamente — é o comportamento esperado, mas não documentado.

**Implicação:** o `#[instrument]` é o "aspecto pronto" do demo, aplicado aos passos de IO simulados. Ele já resolve o caso comum sem escrever macro.

**Fontes:** docs.rs/tracing 0.1.44 — `attr.instrument.html`, `trait.Instrument.html`; docs.rs/tracing-attributes 0.1.31 — `attr.instrument.html` (MSRV 1.65).
**Confiança:** alta para campos, suporte e limitações; média para a expansão interna em `async fn`.

### 4. Padrão de macro de atributo própria para funções sync e async

**Resposta:** a macro precisa viver em uma crate `proc-macro = true`, na raiz, como `pub fn` com assinatura `fn(TokenStream, TokenStream) -> TokenStream`, ABI Rust; ela **não pode ser usada na própria crate** que a define. O segundo `TokenStream` recebe o item inteiro, **incluindo os demais atributos**, e a saída substitui o item.

O padrão para suportar async — parsear `ItemFn`, preservar assinatura/corpo/atributos e, quando `sig.asyncness.is_some()`, envolver o future resultante com `Instrument` — **não está documentado nas fontes oficiais lidas**. É padrão de comunidade; a demo deve validá-lo com teste, não presumi-lo.

Armadilhas documentadas: macros procedurais são "unhygienic" (usar caminhos absolutos e nomes improváveis), e a função da macro em si não pode ser `async`.

**Implicação:** a `aop-macros` usa `syn 3` com feature `full` (`ItemFn`), `quote` e `proc-macro2`. O suporte a async é requisito da demo e precisa de teste específico.

**Fontes:** Rust Reference, `procedural-macros.html`; docs.rs/syn 3.0.10; release notes do syn 3.0.0.
**Confiança:** alta para as restrições de crate e sintaxe; **média** para o padrão async (não documentado oficialmente — validar em código).

### 5. O timing de spans serve para medir blocos CPU-bound?

**Resposta:** sim, com a semântica certa. O busy time é o tempo em que o span esteve *entered*; o idle time é o tempo em que ele existiu sem estar entered. Em um `async fn` instrumentada, o span é entered a cada poll: **busy ≈ tempo de polling (CPU)** e **idle ≈ tempo aguardando (IO/await)**. Para um bloco síncrono dentro de um span entered, o busy acompanha o tempo de execução do bloco.

Limites honestos:
- O tempo é wall-clock dos eventos de enter/exit; a precisão é a do subscriber, não uma medida de CPU do SO.
- A semântica de busy/idle com **spans aninhados** não está documentada nessas páginas; não trate o busy de um span pai como "tempo próprio". O demo mede cada etapa diretamente.
- O overhead do `#[instrument]` não foi medido; fica fora do escopo salvo pedido explícito.

**Fontes:** docs.rs/tracing-subscriber 0.3.23 (`SubscriberBuilder::with_span_events`, `FmtSpan::FULL`); docs.rs/tracing 0.1.44 (`Instrument`).
**Confiança:** alta para as definições de busy/idle; média para a interpretação em spans aninhados.

### 6. `aspect-rs` — evidência do ecossistema

**Resposta:** `aspect-rs.org` é um mdBook real de um framework de AOP para Rust, com macro de atributo `#[aspect(...)]` e trait `Aspect` (`before`/`after`/`after_error`/`around`), weaving em tempo de compilação. Estado em 2026-11-18: crates `aspect-core`/`aspect-macros`/`aspect-std` na linha **0.1.x** (0.1.2 publicado no docs.rs), repositório `github.com/yijunyu/aspect-rs`, MIT OR Apache-2.0, MSRV 1.70+, **5 commits** (17/02–21/05/2026), 18 estrelas, **sem releases/tags**, API declaradamente instável (v1.0 projetada para ~12 meses).

Para o nosso problema: "async fn" é declaradamente suportado pela macro, mas o trait `Aspect` publicado é síncrono e "async aspects" está no roadmap; **não há integração com `tracing`/OpenTelemetry** (aparece no roadmap de longo prazo). Há `TimingAspect`/`MetricsAspect` próprios. As alegações de overhead/testes são do projeto, não verificadas independentemente.

**Implicação:** não vira dependência do demo. Entra no README como comparação do ecossistema, e a razão fica registrada: sem integração com tracing, um `Aspect` próprio que emitisse spans daria o mesmo trabalho da macro própria, com a desvantagem da API 0.1.x sem releases.

**Fontes:** aspect-rs.org (Introduction, Getting Started, Advanced, Roadmap, Troubleshooting); GitHub yijunyu/aspect-rs (commits, tags); docs.rs/aspect-core. Consultadas via subagente web-fetch em 2026-11-18.
**Confiança:** média-alta — documentação do próprio projeto e números do GitHub no momento da consulta; nenhuma verificação independente de performance.

## Recomendação

1. **Versões a fixar no workspace:** `tracing = "0.1.44"`, `tracing-subscriber = "0.3.23"` (default features), `tokio = "1.54.1"` (features `macros`, `rt-multi-thread`, `time`), `syn = "3.0.10"` (`full`), `quote = "1.0.47"`, `proc-macro2 = "1.0.107"`; edição 2021; `rust-version = "1.85"`.
2. **Saída humana:** `with_span_events(FmtSpan::CLOSE)` com timestamps habilitados — é o que faz a duração por etapa aparecer no terminal.
3. **Testes:** camada de captura própria, porque os eventos de close do fmt não são eventos `tracing` reais.
4. **Macro:** `syn 3` + `quote`; async via `Instrument`, com teste específico porque o padrão não é documentado oficialmente.
5. **aspect-rs:** apenas README, sem dependência.
6. **Bloqueio de ambiente:** **não há toolchain Rust instalada neste host** (`rustc`, `cargo`, `rustup` ausentes). O demo não compila nem roda até instalar Rust; a instalação (ex.: rustup) é mudança de sistema e precisa do sim do Operador antes do coder/verify.

## Questões residuais

- A expansão exata de `#[instrument]` em `async fn` (se usa `Instrument` internamente) não é documentada; validar na prática com um teste que capture enter/exit por poll.
- Semântica de busy/idle com spans aninhados não documentada; medir cada etapa diretamente e não inferir "tempo próprio" do span pai.
- Nomes literais dos campos de duração no output do fmt (`time.busy`/`time.idle`) não confirmados nos docs; confirmar na primeira execução.
- Toolchain Rust ausente no host — decisão de instalação pendente com o Operador.
- Overhead do `#[instrument]` não medido; fora do escopo do demo.
