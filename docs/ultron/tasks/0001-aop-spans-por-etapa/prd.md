# PRD — Demonstração de AOP em Rust com spans por etapa

| Campo | Conteúdo |
|---|---|
| **Documento** | PRD-0001 · Product Requirements Document |
| **Projeto** | rust-aop-logging |
| **Autor** | ULTRON · skill ultron-prd — Operador: J Henrique Costa |
| **Data** | 2026-11-18 |
| **Versão do documento** | 1.1 |
| **Status** | draft |
| **Rastreabilidade** | Task 0001-aop-spans-por-etapa · branch `feature/0001-aop-spans-por-etapa` |

> **Uma linha:** um workspace Rust que mostra o mesmo pipeline em dois modos — spans de IO automáticos com os passos CPU invisíveis (baseline) e um span por etapa via macro de atributo (aop) — para medir o tempo de passos CPU-bound que não fazem IO.

## Declaração do problema

Um serviço Rust em produção acumula etapas de processamento dentro de um único span. Os passos que fazem IO ganham span próprio porque a instrumentação automática cobre middleware e clientes; os passos CPU-bound não fazem IO e ficam invisíveis no trace. Quando a latência sobe, o time sabe que a requisição está lenta, mas não qual etapa consome o tempo. A correção precisa ser replicável sem espalhar chamadas de telemetria pelo corpo de cada função.

## Solução

Um workspace Rust que demonstra instrumentação orientada a aspectos:

- `#[instrument]` do `tracing` nos passos que representam IO já instrumentado (clientes simulados).
- Uma macro de atributo própria, `#[aop_span]`, declarada em uma crate `proc-macro`, aplicada aos passos CPU-bound.
- O mesmo pipeline executado em dois modos: `baseline` (span externo e spans de IO automáticos; os passos CPU sem span próprio, reproduzindo a produção) e `aop` (um span por etapa).
- Saída de terminal com `FmtSpan::CLOSE`, mostrando a duração de cada span (tempo ocupado e ocioso).
- README que explica o que é AOP em Rust, mostra o antes/depois, compara com `aspect-rs` e aponta o caminho OTLP para produção.

## Histórias de usuário

1. Como desenvolvedor Rust que mantém um serviço com latência alta, quero ver cada etapa do processamento em seu próprio span, para identificar qual passo consome o tempo.
2. Como desenvolvedor, quero aplicar a instrumentação por atributo na função, sem espalhar chamadas de span pelo corpo, para manter a lógica de negócio limpa.
3. Como desenvolvedor, quero um modo `baseline` que reproduza o problema, para comparar antes e depois no mesmo projeto.
4. Como desenvolvedor, quero que a macro funcione em funções `async`, para instrumentar handlers e passos assíncronos.
5. Como SRE de plantão, quero ver a duração de cada etapa no terminal, para diagnosticar sem depender de um coletor externo.
6. Como SRE, quero distinguir tempo ocupado (CPU) de tempo ocioso (espera), para separar processamento de IO.
7. Como tech lead, quero um README que explique o que é AOP em Rust e compare com `aspect-rs`, para avaliar a adoção no time.
8. Como tech lead, quero saber como levar o mesmo padrão para produção com OTLP, para não ficar preso ao formatter de terminal.
9. Como desenvolvedor, quero testes que provem que os spans existem no modo `aop` e não no `baseline`, para evitar regressão silenciosa.
10. Como novo integrante do time, quero rodar o demo com um comando, para entender o padrão em minutos.
11. Como mantenedor, quero as versões das crates fixadas e documentadas, para builds reproduzíveis.
12. Como desenvolvedor, quero que a instrumentação preserve o retorno, os erros e o comportamento assíncrono da função, para não alterar semântica ao medir.
13. Como desenvolvedor, quero escolher o nome do span criado pela macro, para alinhar a nomenclatura com a do sistema real.
14. Como revisor, quero que o PRD registre as decisões de escopo (sem HTTP, sem OTLP implementado, sem `aspect-rs` como dependência), para não haver dúvida sobre o que o demo entrega.

## Decisões de implementação

- **Estrutura:** workspace Cargo com duas crates — `aop-macros` (proc-macro) e `demo` (binário).
- **Pipeline simulado:** `handle` (span externo) → `validate` (CPU) → `load_customer` (IO simulado) → `price` (CPU) → `apply_rules` (CPU) → `persist` (IO simulado) → `render` (CPU).
- **Separação baseline/aop sem duplicar lógica:** cada etapa tem um corpo puro (`validate_impl`, `price_impl`, …) e uma versão fina anotada (`#[aop_span] fn validate(...)`), que delega ao corpo. O modo `baseline` chama os corpos puros; o modo `aop` chama as versões anotadas. A diferença entre os modos é exatamente a camada de aspecto.
- **IO simulado:** `tokio::time::sleep` em clientes falsos (`CustomerClient`, `OrderRepository`), anotados com `#[instrument]` — representam o que a instrumentação automática já entrega em produção.
- **CPU simulada:** trabalho determinístico com contagem de iterações ajustável, sem `sleep`, para as etapas terem duração mensurável.
- **Macro `#[aop_span]`:** cria um span no nível `INFO` com o nome da função; em `async fn`, instrumenta o future com `tracing::Instrument`; emite um evento de conclusão com o tempo decorrido; aceita `name = "..."` para renomear o span. Suporta funções síncronas e assíncronas.
- **Saída:** `tracing_subscriber::fmt` com `with_span_events(FmtSpan::CLOSE)` e timestamps habilitados — é o que faz a duração aparecer. OTLP fica documentado como caminho de produção, não implementado.
- **Versões (fonte: research 0001):** `tracing 0.1.44`, `tracing-subscriber 0.3.23`, `tokio 1.54.1` (features `macros`, `rt-multi-thread`, `time`), `syn 3.0.10` (feature `full`), `quote 1.0.47`, `proc-macro2 1.0.107`; edição 2021; `rust-version = "1.85"` (MSRV do tokio).
- **Pré-requisito de ambiente:** o host não tem toolchain Rust (`rustc`, `cargo`, `rustup` ausentes). A instalação é necessária antes do `coder`/`verify` e depende de aprovação do Operador.
- **Tracker local:** o PRD vive em `docs/ultron/tasks/0001-aop-spans-por-etapa/prd.md`; no tracker local, o `task.md` é a issue. Nada é publicado em serviço externo.
- **README:** o que é AOP em Rust; como a macro expande; trace `baseline` vs `aop`; comparação com `aspect-rs` (0.1.x, 5 commits, sem releases, sem integração com tracing/OpenTelemetry — ver research 0001); nota de OTLP.

## Decisões de teste

- **Testar comportamento externo, não implementação:** os testes observam os spans emitidos (nome, pai/filho, ciclo de vida) e o valor de retorno das funções instrumentadas; não inspecionam o texto da expansão da macro.
- **Seams de teste:**
  1. `demo::handle` (pipeline público) — seam end-to-end usado pelo binário e pelos testes do pipeline.
  2. `aop_macros::aop_span` — seam da macro, exercitado em funções síncronas e assíncronas de teste.
  3. O subscriber — o binário instala o `fmt`; os testes instalam uma `Layer` de captura via `tracing::subscriber::with_default`.
  4. Os clientes falsos (`CustomerClient`, `OrderRepository`) — seam que simula o IO e carrega o `#[instrument]`.
- **Captura própria:** os testes não podem depender dos eventos sintetizados por `FmtSpan::CLOSE`, porque eles não são eventos `tracing` reais (research 0001, pergunta 2). A camada de captura registra criação, entrada e fechamento dos spans.
- **Cenários:** modo `aop` emite um span por etapa com a hierarquia correta sob o span de `handle`; modo `baseline` emite apenas o span externo; a macro preserva retorno, propagação de erro e comportamento async.
- **Prior art:** nenhum — o repositório não tem código anterior; os testes nascem com o projeto.

## Avaliação de dependências (SCA)

Sem lockfile nesta data — o workspace ainda não existe. Após o primeiro build (`Cargo.lock`), rodar `/ultron-deps-audit` e registrar o snapshot. O conjunto de dependências é pequeno e fixado por este PRD.

## Fora de escopo

- Alterar o sistema de produção (outro repositório).
- Servidor HTTP (axum, tower ou equivalente); o demo simula a requisição em um binário.
- Exportação OTLP/Jaeger implementada; fica documentada como caminho de produção.
- Dependência de `aspect-rs`; ele aparece apenas na comparação do README.
- Benchmark do overhead da macro.
- Outras linguagens ou stacks de observabilidade.

## Notas adicionais

- A investigação do `aspect-rs` está em `research.md`, pergunta 6; o veredito é comparação documental, não dependência.
- A toolchain Rust ausente é o único bloqueio de execução conhecido; todo o restante é código novo em um repositório vazio.
- O demo é didático: o domínio é ilustrativo e o produto é o padrão de instrumentação, não a regra de negócio.

## Critérios de aceitação

- [AC1] `cargo run -- baseline` executa o pipeline e imprime o **span externo e os spans de IO** automáticos; nenhum span de etapa CPU aparece.
- [AC2] `cargo run -- aop` executa o mesmo pipeline e imprime **um span por etapa** (`validate`, `load_customer`, `price`, `apply_rules`, `persist`, `render`).
- [AC3] Em ambos os modos, a saída mostra a duração de cada span (campos de tempo ocupado e ocioso) quando há timestamps.
- [AC4] As etapas CPU-bound são instrumentadas por `#[aop_span]`, sem chamadas manuais de `span!`/`info_span!` dentro dos corpos de `validate`, `price`, `apply_rules` e `render`.
- [AC5] Os testes de captura provam a presença dos spans por etapa no modo `aop` e a ausência deles no `baseline`.
- [AC6] A macro funciona em `fn` e `async fn`, preservando retorno e propagação de erro; há teste para cada caso.
- [AC7] O README explica AOP em Rust, mostra o antes/depois, compara com `aspect-rs` e descreve o caminho OTLP de produção.
- [AC8] O workspace compila com as versões fixadas, edição 2021 e `rust-version = "1.85"`.

---

## Proveniência

Descoberta da task 0001 (2026-11-18) e `research.md` da mesma task — versões do crates.io, semântica de `FmtSpan::CLOSE` e `#[instrument]` (docs.rs), restrições de proc-macro (Rust Reference) e evidência de `aspect-rs` (aspect-rs.org, GitHub, docs.rs).

## Histórico de versões

| Versão | Data | Autor | Mudança |
|---|---|---|---|
| 1.1 | 2026-11-18 | ULTRON · ultron-slices | AC1 ajustado: baseline fiel à produção (IO instrumentado, CPU invisível) |
| 1.0 | 2026-11-18 | ULTRON · ultron-prd | Versão inicial, a partir da descoberta e da research 0001 |

## Rastreabilidade

Task 0001-aop-spans-por-etapa › PRD-0001

<sub>Gerado por ULTRON v2.1.1 · skill ultron-prd · 2026-11-18T14:56:44-03:00 · em conformidade com a Convenção de Documento ULTRON. Documento versionado em git — a proveniência viaja com o projeto.</sub>
