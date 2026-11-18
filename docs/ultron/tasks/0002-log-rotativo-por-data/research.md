# Research 0002 — Log rotativo por data com `tracing-appender`

**Data:** 2026-11-18
**Contexto:** task 0002-log-rotativo-por-data (feature) — questões abertas da descoberta
**Context7:** indisponível — o MCP deste ambiente exige autenticação OAuth; doc de biblioteca obtida das páginas oficiais (docs.rs/tracing.rs), como manda a degradação graciosa
**Tavily:** indisponível (`TAVILY_API_KEY` ausente) — pesquisa via `web_search` + fetch

## Perguntas investigadas

### 1. Versão atual, MSRV e compatibilidade com o workspace

**Resposta:** a versão mais recente é **`tracing-appender 0.2.5`**, publicada em **2026-04-17**. O `rust-version` declarado é **1.63.0** (edição 2018). O crate não tem tabela `[features]` própria no `Cargo.toml` — só a feature implícita `parking_lot`, decorrente de uma dependência opcional, desabilitada por padrão; não é preciso declarar features no workspace. Dependências relevantes: `crossbeam-channel ^0.5.6`, `thiserror ^2`, `time ^0.3.2` (features `formatting`/`parsing`, sem defaults), `symlink ^0.1.0`. As dev-dependencies citam `tracing ^0.1.35` e `tracing-subscriber ^0.3.18`.

Para o workspace da demo: `tracing 0.1.44` satisfaz `^0.1.35`, `tracing-subscriber 0.3.23` satisfaz `^0.3.18`, e o MSRV de 1.63 está abaixo do `rust-version = "1.85"` já fixado (ditado pelo tokio). **Compatível.**

**Fontes:**
- docs.rs — `tracing-appender 0.2.5`, `Cargo.toml` (primária, publicada em 2026-04-17)
- docs.rs — feature flags da 0.2.5 (primária)
- Changelog da crate no docs.rs (primária; 0.2.3 subiu o MSRV para 1.63)

**Confiança:** alta. Uma página auxiliar (DeepWiki) lista MSRV 1.65 para o workspace, em conflito com o changelog; ambos são irrelevantes para o nosso piso de 1.85.

### 2. Fuso e formato do nome na rotação diária

**Resposta:** a data é anexada **em UTC**, e isso é fixo. A documentação do `RollingFileAppender` diz literalmente que ele "will automatically append the current date and hour (UTC format) to the file name". O builder oferece `rotation`, `filename_prefix`, `filename_suffix`, `latest_symlink` e `max_log_files` — **não há opção de formato de data nem de fuso local**. Na rotação diária, o nome é `[prefixo.]yyyy-MM-dd[.sufixo]`; com `rolling::daily(dir, "demo.log")` o arquivo é `demo.log.2026-11-18`.

**Implicação:** o requisito da descoberta ("a data do nome é a local") **não é atendido pela crate**. O fork consciente registrado na descoberta fica, então, entre (a) aceitar UTC e documentar o deslocamento — em UTC-3 o arquivo "de 18/11" começa às 21:00 do dia 17 local — ou (b) escrever uma rotação própria com data local, o que significa abrir mão do `RollingFileAppender` e implementar troca de arquivo/limpeza à mão.

**Fontes:**
- docs.rs — `RollingFileAppender` 0.2.5 (primária; frase do UTC)
- docs.rs — `rolling::Builder` 0.2.5 (primária; métodos disponíveis, sem fuso/formato)
- tracing.rs — `rolling` (primária)

**Confiança:** alta para o UTC fixo e o formato do nome; alta para a ausência de opção de fuso.

### 3. Retenção nativa

**Resposta:** existe retenção nativa por contagem: `RollingFileAppender::builder().max_log_files(n)`. O appender mantém no máximo `n` arquivos, apagando os mais antigos ao construir ou ao iniciar um novo arquivo. O default é **sem limite** (`None`); `0` desabilita a remoção. Duas ressalvas documentadas: a contagem retida pode às vezes ficar **abaixo** do máximo — "if you need to retain `m` log files, specify a max of `m + 1`" — e os candidatos à remoção são filtrados por prefixo/sufixo quando declarados, ou por nome parseável como data quando não há nenhum dos dois. A retenção é **por contagem de arquivos**, não por tamanho: um único dia pode crescer sem teto.

**Implicação:** a decisão "manter os últimos 7" é implementável com `.max_log_files(7)`, aceitando a ressalva de às vezes reter 6; se o requisito for "pelo menos 7", usar 8. Recomendo `7` como **teto**, que é o que a descoberta pediu, e registrar a ressalva no README.

**Fontes:**
- docs.rs — `rolling::Builder` 0.2.5 (primária)
- docs.rs — `RollingFileAppender` 0.2.5 (primária)

**Confiança:** alta.

### 4. Semântica do `non_blocking` + `WorkerGuard`

**Resposta:** `tracing_appender::non_blocking(writer)` devolve `(NonBlocking, WorkerGuard)`. O `WorkerGuard` "flushes spans/events associated to a `NonBlocking` on a drop" e existe para garantir que **todos** os logs em buffer sejam escritos. Ele deve ser ligado a uma **variável nomeada** — `_` faz o guard ser dropado imediatamente e a garantia se perde — e mantido no `main`/entrypoint. O flush acontece no retorno normal do `main` ou durante unwinding; **não** acontece em encerramento abrupto, e a doc cita explicitamente `std::process::exit` como caso em que "some spans or events may not be written".

A fila é **lossy por padrão**: `NonBlockingBuilder` usa `lossy(true)` e buffer de 128.000 linhas; ao encher, linhas são descartadas. Com `lossy(false)`, os emissores sofrem backpressure. O drop do guard usa timeout de 100 ms para enviar o shutdown e espera até 1000 ms pelo flush. Não existe `WorkerGuard::flush()` explícito na 0.2.5 — a issue tokio-rs/tracing#3267 pede o método e segue **aberta**, sem PR.

**Implicação para o demo:** o `main` atual chama `std::process::exit(1)` no caminho de erro **depois** de o pipeline ter logado. Esse caminho pode perder as últimas linhas do arquivo. A implementação precisa trocar o `exit` por retorno de `std::process::ExitCode` (ou dropar o guard antes de sair), mantendo o guard vivo até o fim. Os testes executam caminhos de sucesso (retorno normal), então leem o arquivo depois do processo terminar, com o buffer esvaziado.

**Fontes:**
- docs.rs — `non_blocking::WorkerGuard` 0.2.5 (primária)
- docs.rs — `NonBlockingBuilder` 0.2.5 (primária; lossy e limite de buffer)
- docs.rs — fonte de `non_blocking.rs` 0.2.5 (primária; timeouts do drop)
- GitHub tokio-rs/tracing#3267 (primária; issue aberta em 2025-05-03)

**Confiança:** alta.

### 5. Composição de dois `fmt` layers (terminal + arquivo)

**Resposta:** o padrão canônico é um `registry()` com dois `fmt::Layer`, cada um com seu writer:

```rust
let file_appender = tracing_appender::rolling::daily(dir, "demo.log");
let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

let subscriber = tracing_subscriber::registry()
    .with(fmt::Layer::new().with_writer(std::io::stdout)
        .with_span_events(FmtSpan::CLOSE))
    .with(fmt::Layer::new().with_writer(non_blocking)
        .with_ansi(false)
        .with_span_events(FmtSpan::CLOSE));
```

`fmt::Layer::with_span_events(FmtSpan)` existe e aceita `FmtSpan::CLOSE`; os eventos sintetizados de close — com `time.busy`/`time.idle` quando há timestamps — aparecem **só no output daquele formatter**, o que é exatamente o que queremos: cada sink recebe o mesmo conjunto de eventos. `with_ansi(false)` no layer do arquivo evita códigos de escape.

Há uma ressalva de comunidade: com layers ANSI e não-ANSI no mesmo registry, uma versão antiga do `tracing-subscriber` podia cachear um único modo de ANSI (discussão #3229, correção citada em PR #3221). Não confirmei em qual versão a correção entrou; a nossa é a **0.3.23**, e o teste que afirma "arquivo sem ANSI" cobre a regressão. Se ela aparecer, a ordem do layer do arquivo antes do de stdout é o contorno conhecido. O `tracing-subscriber 0.3.23` também traz a mudança de sanitização de ANSI (#3484); o terminal da demo continua colorido como hoje.

**Fontes:**
- tokio-rs/tracing — exemplo oficial `fmt-multiple-writers.rs` (primária)
- docs.rs — `fmt::Layer::with_span_events` (primária)
- docs.rs — `FmtSpan` (primária; `CLOSE` com busy/idle)
- GitHub tokio-rs/tracing — discussão #3229 (secundária/comunidade; ressalva de ANSI)
- Changelog do `tracing-subscriber` 0.3.23 (primária)

**Confiança:** alta para o padrão de composição; média para a ressalva de ANSI (não confirmada na 0.3.23, coberta por teste).

## Recomendação

A evidência sustenta esta configuração para a demo:

- `tracing-appender = "=0.2.5"`, sem features, coerente com o estilo de pins do workspace.
- **Aceitar UTC e documentar** o deslocamento, em vez de escrever rotação própria. O demo é sobre AOP; uma rotação local artesanal abriria um segundo assunto no código e no README. O leitor do arquivo vê a data UTC no nome e a nota explica o recorte.
- `.max_log_files(7)` como teto, com a ressalva de contagem no README.
- `non_blocking` com o guard no `main`; substituir o `std::process::exit(1)` do caminho de erro por retorno de `ExitCode` para não perder as últimas linhas.
- Dois `fmt::Layer` (stdout e arquivo), ambos com `FmtSpan::CLOSE`; `.with_ansi(false)` só no arquivo; filtro de nível `INFO` no registry ou por layer.
- `logs/` no `.gitignore`; `DEMO_LOG_DIR` para os testes, com `./logs` como padrão.

O que a research informa e o PRD decide: a aceitação do UTC é uma decisão de escopo — a alternativa (rotação local própria) continua disponível, mas tem custo desproporcional ao demo.

## Questões residuais

- A versão exata em que a correção de cache de ANSI entrou não foi confirmada; o teste de "arquivo sem ANSI" é a salvaguarda, e a ordem dos layers é o contorno conhecido.
- O `Cargo.toml` lido da 0.2.5 lista `tracing-subscriber ^0.3.18` entre as dependências; não foi possível distinguir com certeza dependência normal de dev-dependency na leitura da página. Em qualquer caso, a faixa é satisfeita pela 0.3.23 do workspace.
- A rotação acontece na virada UTC (00:00 UTC = 21:00 em UTC-3); a doc não detalha o instante exato em que o appender troca de arquivo, apenas que a data é UTC. O README deve dizer o fuso, não prometer um horário local exato.
- Retenção por tamanho não existe na crate; se um dia for necessária, é outra decisão (fora de escopo).
