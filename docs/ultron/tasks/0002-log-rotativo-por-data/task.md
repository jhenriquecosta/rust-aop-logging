# Task 0002-log-rotativo-por-data — Log rotativo por data na demo de AOP

tipo: feature
branch: feature/0002-log-rotativo-por-data
estado: em andamento
worktree: .claude/worktrees/feature-0002-log-rotativo-por-data
responsavel: jhenriquecosta@hotmail.com
next_skill: ultron-coder

Além do trace no terminal, a demo passa a gravar o log em arquivo com rotação por data. A motivação é ter o trace persistido para análise posterior sem depender de um coletor. Os detalhes — granularidade da rotação (diária), nome e diretório do arquivo, retenção, formato e se o stdout continua — ficam para a descoberta.

## Decisões da descoberta (2026-11-18)

- **Escopo do arquivo:** o mesmo stream que o terminal já recebe — spans e eventos de `close` com `time.busy`/`time.idle` — nos dois modos (`baseline` e `aop`). O terminal continua ativo em paralelo; o arquivo é o mesmo trace, persistido.
- **Rotação e local:** diária, por data. Diretório `logs/` na raiz do projeto (ignorado no git); o nome carrega a data. O diretório é sobrescrito por `DEMO_LOG_DIR` para os testes; o padrão `./logs` vale para quem roda a demo.
- **Fuso da data:** a data do nome é a **local**. Se a crate só rotacionar em UTC, a decisão vira fork consciente: aceitar UTC documentado ou escrever rotação própria. A research confirma.
- **Retenção:** manter os últimos **7 arquivos diários**, removendo os mais velhos. Se a crate não oferecer retenção nativa, decidir entre limpeza própria (~10 linhas) ou não apagar e documentar.
- **Writer:** **não bloqueante** (`tracing_appender::non_blocking`), com `WorkerGuard` mantido no `main` até o fim. Gravar no thread medido contaminaria o tempo que a demo existe para mostrar. O risco de perder as últimas linhas se o guard cair cedo fica documentado.
- **Formato do arquivo:** mesmo formato do terminal, **sem ANSI**.
- **Testes:** teste de integração por modo, executando o binário com `DEMO_LOG_DIR` num diretório temporário; afirma que o arquivo com a data local existe, que contém os spans do modo, que o terminal continua recebendo o trace e que não há ANSI no arquivo. A leitura acontece depois que o processo termina (o guard esvazia o buffer). Retenção só é testada se a limpeza for nossa.
- **README:** seção curta com diretório, rotação, retenção, `DEMO_LOG_DIR`, semântica do guard e a ressalva de fuso. `logs/` no `.gitignore`.

## Fora de escopo

- Formato JSON/estruturado, compressão, envio para coletor (OTLP continua só documentado), rotação por tamanho, configuração de nível por sink e formatter próprio.
- A demo continua rodando com um comando, sem variável de ambiente obrigatória.

## Questões abertas (para /ultron-research)

- Versão atual do `tracing-appender`, MSRV e features; compatibilidade com `rust-version = "1.85"` e com `tracing 0.1.44` / `tracing-subscriber 0.3.23`.
- Fuso e formato exato do nome produzido pela rotação diária (`UTC`? prefixo + data?); se há opção de data local.
- Suporte nativo a retenção (`max_log_files` ou equivalente) e como ela conta os arquivos.
- Semântica do `non_blocking` + `WorkerGuard`: o que garante o flush no encerramento e o que se perde num `exit` abrupto.
- Como compor dois `fmt` layers (terminal + arquivo) com `FmtSpan::CLOSE`, ANSI desligado só no arquivo, e o `init()` global trocado por `registry().with(...)`.

## Fatias publicadas

| # | Fatia | Issue | Estado |
|---|---|---|---|
| 1 | Arquivo diário no binário | — (local) | concluída · verificada 2026-11-18 |
| 2 | Retenção de 7 | — (local) | concluída · verificada 2026-11-18 |
| 3 | README e fuso documentado | — (local) | publicada |

Os corpos estão em `slices/` (fatias 01–03), na ordem de execução.

## Critérios de aceitação (do PRD-0002)

- [x] AC1 — com `DEMO_LOG_DIR`, o `baseline` cria `demo.log.<data UTC do dia>` no diretório.
- [x] AC2 — o arquivo do `baseline` tem `handle`, `load_customer` e `persist`, sem spans de CPU.
- [x] AC3 — o arquivo do `aop` tem as seis etapas sob `handle`, com `time.busy`/`time.idle`.
- [x] AC4 — o terminal continua recebendo o trace nos dois modos.
- [x] AC5 — o arquivo não contém ANSI.
- [x] AC6 — com 8 arquivos diários pré-existentes, uma execução deixa no máximo 7 (a poda é por btime; com um arquivo por dia, o mais antigo sai).
- [x] AC7 — sem `DEMO_LOG_DIR`, os arquivos vão para `logs/`; `logs/` está no `.gitignore`.
- [ ] AC8 — o README documenta diretório, rotação UTC, retenção, `DEMO_LOG_DIR`, guard e execução.
- [x] AC9 — `tracing-appender =0.2.5` fixado; build, clippy e a suíte seguem verdes.

## Exceções registradas

- Portão `logging` (médio) do coder desviado em 2026-11-18: o projeto não tem `docs/ultron/agents/logging.md`, mas o PRD-0002 fixa a configuração de logging da feature (dois sinks, `FmtSpan::CLOSE`, retenção de 7, writer não bloqueante); uma convenção separada duplicaria a decisão. Aprovado pelo Operador.
- Portão `verify` do arsenal (2026-11-18): o `verify_all.sh` não tem ramo Rust e sai `2` (nada verificado). A fatia 1 foi verificada manualmente: `cargo build`, `cargo clippy --workspace --all-targets -- -D warnings` e `cargo test` — 15 testes verdes.
