# rust-aop-logging

> Demonstração de instrumentação orientada a aspectos em Rust: o mesmo pipeline em dois modos — `baseline`, com os passos CPU invisíveis dentro de um único span, e `aop`, com um span por etapa.

---

## O que é

Um serviço async simulado que reproduz um problema de produção: os passos que fazem IO ganham span próprio porque a instrumentação automática cobre middleware e clientes; os passos CPU-bound não fazem IO e desaparecem dentro do span do handler. Quando a latência sobe, o trace mostra que a requisição está lenta — não qual etapa consome o tempo.

O projeto mostra o antes e o depois no mesmo código. Os passos de IO usam o `#[instrument]` do `tracing`, que é o aspecto pronto que a instrumentação de cliente já oferece. Os passos CPU recebem uma macro de atributo própria, `#[aop_span]`, que abre um span, mede o tempo e fecha, sem espalhar chamadas de telemetria pelo corpo da função.

**Stack:** Rust 1.85+ (testado com 1.100) · tokio 1.54 · tracing 0.1.44 · tracing-subscriber 0.3.23 · tracing-appender 0.2.5 · syn 3.0

---

## Antes de começar

A toolchain Rust é o único pré-requisito. Não há serviço externo, banco ou coletor para subir.

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal
```

---

## Como rodar

```bash
cargo run -- baseline   # reproduz o problema: IO visível, CPU invisível
cargo run -- aop        # a solução: um span por etapa
cargo test              # a suíte que prova os dois modos
```

**Como saber que deu certo:** o modo `baseline` imprime o span `handle` e os spans de IO (`load_customer`, `persist`), sem nenhum span de etapa CPU. O modo `aop` imprime os seis spans de etapa sob `handle`.

Os dois comandos também gravam o trace em `logs/demo.log.<data>` no diretório de onde você executa. O terminal e o arquivo recebem o mesmo trace; a seção [Log em arquivo](#log-em-arquivo) detalha caminho, rotação e retenção.

---

## Como usar

O pipeline é o mesmo nos dois modos:

| Etapa | Tipo | Instrumentação |
|---|---|---|
| `validate` | CPU | `#[aop_span]` (modo `aop`) |
| `load_customer` | IO simulado | `#[instrument]` (aspecto pronto) |
| `price` | CPU | `#[aop_span]` |
| `apply_rules` | CPU | `#[aop_span]` |
| `persist` | IO simulado | `#[instrument]` |
| `render` | CPU | `#[aop_span]` |

**`baseline`** — o problema, em três linhas (durações variam a cada execução):

```
INFO handle:load_customer{id=1}: demo: close time.busy=32.1µs time.idle=21.3ms
INFO handle:persist: demo: close time.busy=39.5µs time.idle=20.4ms
INFO handle: demo: close time.busy=325µs time.idle=41.6ms
resposta: Response { customer_name: "Cliente 1", total_cents: 2500, items_count: 3 }
```

**`aop`** — a solução, com cada etapa no seu span:

```
INFO handle:validate: demo: close time.busy=2.37µs time.idle=2.94µs
INFO handle:load_customer{id=1}: demo: close time.busy=36.9µs time.idle=24.6ms
INFO handle:price: demo: close time.busy=2.73µs time.idle=3.73µs
INFO handle:apply_rules: demo: close time.busy=1.32µs time.idle=3.19µs
INFO handle:persist: demo: close time.busy=32.2µs time.idle=21.0ms
INFO handle:render: demo: close time.busy=913ns time.idle=2.16µs
INFO handle: demo: close time.busy=393µs time.idle=45.5ms
resposta: Response { customer_name: "Cliente 1", total_cents: 2500, items_count: 3 }
```

**Como ler os números:** `time.busy` é o tempo em que o span esteve entered — para um passo síncrono, o tempo de execução; em uma `async fn` instrumentada, o tempo de polling (CPU). `time.idle` é o tempo em que o span existiu sem estar entered — a espera de IO. O `busy` do `handle` inclui o `busy` dos filhos; para o tempo de cada etapa, leia o span da própria etapa.

---

## Log em arquivo

Os dois modos gravam o trace em um arquivo diário, além do terminal. A escrita acontece em uma thread separada: o pipeline mede o trabalho, não o disco.

**Onde:** por padrão, `logs/` no diretório de onde você executa o binário. `DEMO_LOG_DIR` troca o destino.

| Variável | Obrigatória | Padrão | O que faz |
|---|---|---|---|
| `DEMO_LOG_DIR` | não | `logs/` (relativo ao diretório atual) | diretório dos arquivos de log |

```bash
cargo run -- baseline                      # grava em ./logs/
DEMO_LOG_DIR=/tmp/trace cargo run -- aop   # grava em /tmp/trace/
ls logs/                                   # demo.log.2026-11-18
```

**Nome e rotação:** `demo.log.yyyy-MM-dd`, um arquivo por dia, com a data em **UTC** — a crate não oferece fuso local. Em UTC-3, o arquivo "de 19/11" começa a ser escrito às 21:00 do dia 18 local e cobre até 20:59 do dia 19.

**Retenção:** no máximo **7 arquivos**. A poda usa o tempo de criação do arquivo (btime); quando o sistema de arquivos não informa btime, usa a data do nome. É um teto por contagem, não por tamanho: um dia movimentado cresce sem limite dentro do próprio arquivo. A contagem retida pode ocasionalmente ficar abaixo do máximo.

**Formato:** o mesmo do terminal, sem códigos ANSI. Trecho real do arquivo (durações variam):

```
2026-11-18T19:35:21.379516Z  INFO handle:load_customer{id=1}: demo: close time.busy=40.6µs time.idle=21.2ms
2026-11-18T19:35:21.401106Z  INFO handle:persist: demo: close time.busy=74.0µs time.idle=21.3ms
```

**Encerramento e saturação:** o writer é não bloqueante. O guard que mantém a fila viva fica no `main` até o fim e esvazia o buffer no drop. A fila é lossy — o buffer padrão é de 128 mil linhas — e, sob saturação, linhas podem ser descartadas. Encerramento abrupto — `std::process::exit` ou abort — pode perder as últimas linhas; o caminho de erro do demo solta o guard antes de sair.

---

## Como funciona a macro

Rust não tem AOP de runtime. O equivalente são macros de atributo, aplicadas em tempo de compilação. O `#[instrument]` do `tracing` já é uma delas — e é o que o demo usa nos passos de IO.

A `#[aop_span]` faz o mesmo recorte, de forma explícita e pequena:

- Em uma `fn`, abre um span no nível `INFO` com o nome da função, entra nele, executa o corpo e fecha no fim.
- Em uma `async fn`, embrulha o future com `tracing::Instrument`, então o span é entered a cada poll — o mesmo mecanismo que o `tracing` usa para instrumentar futures.
- `#[aop_span(name = "preco")]` renomeia o span; sem argumento, o nome é o da função.

A lógica não é duplicada entre os modos: cada etapa tem um corpo puro (`validate_impl`, `price_impl`, …) e uma versão fina anotada, que delega ao corpo. O modo `baseline` chama os corpos; o modo `aop` chama as versões anotadas. A diferença entre os modos é exatamente a camada de aspecto.

---

## Comparação com o `aspect-rs`

O [`aspect-rs`](https://aspect-rs.org/) é o projeto mais próximo de um "framework de AOP" para Rust: uma macro `#[aspect(...)]` com um trait `Aspect` (`before`/`after`/`after_error`/`around`), também com weaving em tempo de compilação. A investigação (em [`research.md`](docs/ultron/tasks/0001-aop-spans-por-etapa/research.md)) mostrou por que ele não é dependência deste demo:

- Os crates estão na linha **0.1.x** (0.1.2 publicado), sem releases ou tags no GitHub, com API declaradamente instável.
- **Não há integração com `tracing` ou OpenTelemetry.** Para emitir spans, seria preciso escrever um `Aspect` próprio — o mesmo trabalho da macro deste demo, com a dependência a mais.
- O trait `Aspect` publicado é síncrono; "async aspects" está no roadmap.

O que os dois compartilham é a ideia: instrumentação declarada no atributo, fora do corpo. O que muda é o acoplamento ao ecossistema de tracing e a maturidade.

---

## Caminho para produção (OTLP)

O demo escreve no terminal e em arquivo com `tracing-subscriber`. Em produção, os mesmos spans seguem para um backend OpenTelemetry: troca-se o subscriber `fmt` por um exporter OTLP (o par `tracing-opentelemetry` + `opentelemetry-otlp` é o caminho comum), e os nomes e a hierarquia dos spans não mudam. O `busy`/`idle` do `fmt` é um detalhe do formatter de terminal; no backend, cada span carrega início, fim e atributos, e a duração por etapa é a diferença entre eles.

Isso não está implementado aqui de propósito: o demo precisa rodar com um comando, sem coletor.

---

## Como testar

```bash
cargo test
```

A suíte cobre os dois modos, a macro e o arquivo:

- `span_tree` — captura os spans com uma `Layer` própria e prova que o baseline não separa as etapas CPU e que o modo `aop` as aninha sob `handle`.
- `aop_macro` — prova a macro em `fn` e `async fn`, o retorno, a propagação de erro e o `name`.
- `aop_mode` / `cli` — executam o binário e conferem a saída dos dois modos.
- `pipeline` — o cálculo do pedido e a propagação de erro do cliente de IO.
- `file_log` — executa o binário em diretórios temporários e prova o arquivo do dia, o conteúdo por modo, a ausência de ANSI, o destino padrão e o teto de 7 arquivos.

A captura nos testes não usa os eventos sintetizados por `FmtSpan::CLOSE`, porque eles não são eventos `tracing` reais — só aparecem no formatter.

---

## Estrutura

```
aop-macros/         a macro de atributo #[aop_span] (crate proc-macro)
demo/src/lib.rs     pipeline, etapas, clientes falsos e os dois modos
demo/src/main.rs    CLI: baseline | aop + subscriber (stdout + arquivo)
demo/tests/         captura de spans, macro, CLI, pipeline e log em arquivo
```

---

## Decisões e limitações

- **Sem servidor HTTP.** O binário simula a requisição; o formato do serviço async é o que importa, não o transporte.
- **IO simulado** com `tokio::time::sleep`; o CPU é trabalho determinístico. Não há rede nem banco.
- **Rotação em UTC.** A data do nome é UTC: em UTC-3, o arquivo do dia D começa às 21:00 do dia D-1 local. A crate não oferece fuso local; uma rotação com data local exigiria um writer próprio.
- **Retenção por contagem.** `max_log_files(7)` limita o número de arquivos, não o tamanho de cada um. A poda é pelo tempo de criação (btime), com fallback para a data do nome.
- **Formato texto, sem compressão.** O arquivo é texto plano — sem JSON/estruturado — e os arquivos antigos não são comprimidos.
- **Writer não bloqueante.** As linhas passam por uma fila; um encerramento abrupto pode perder as últimas. O `main` solta o guard antes do `exit(1)`.
- **`aspect-rs` só na comparação** — não é dependência, pelos motivos acima.
- **OTLP não implementado** — documentado como caminho de produção.
- **Sem benchmark de overhead.** O custo da macro não é medido neste demo.

---

## Manutenção

**Responsável:** Henrique Costa · **Projeto:** rust-aop-logging (tracker local, issues em `docs/ultron/tasks/`)
