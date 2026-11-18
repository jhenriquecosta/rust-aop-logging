# PRD — Log rotativo por data na demo de AOP

| Campo | Conteúdo |
|---|---|
| **Documento** | PRD-0002 · Product Requirements Document |
| **Projeto** | rust-aop-logging |
| **Autor** | ULTRON · skill ultron-prd — Operador: J Henrique Costa |
| **Data** | 2026-11-18 |
| **Versão do documento** | 1.2 |
| **Status** | draft |
| **Rastreabilidade** | Task 0002-log-rotativo-por-data · branch `feature/0002-log-rotativo-por-data` |

> **Uma linha:** a demo passa a espelhar o trace do terminal em um arquivo rotativo diário (`tracing-appender`), com retenção de 7 arquivos, mantendo o `stdout` e sem tocar no pipeline.

## Declaração do problema

Quem diagnostica a demo — ou um serviço que copie o padrão — depende do terminal: o trace some quando o processo termina ou quando o scroll passa. Sem persistência, não há como comparar execuções, revisar um trace depois do fato, ou levar o artefato para outra pessoa. A demo já mostra a instrumentação por etapa; falta um destino durável para o que ela emite.

## Solução

O binário instala **dois sinks do mesmo trace**: o `stdout` que já existe e um arquivo diário em `logs/`, via `tracing-appender`. O arquivo carrega a data no nome, é escrito por um worker não bloqueante — para não contaminar o `time.busy` que a demo existe para mostrar — e mantém no máximo 7 arquivos. O diretório é sobrescrito por `DEMO_LOG_DIR` para os testes. O README documenta o caminho, a rotação, a retenção e o fuso.

## Histórias de usuário

1. Como desenvolvedor depurando a demo, quero o trace também em arquivo, para analisar depois que o processo termina.
2. Como desenvolvedor, quero que o arquivo rotacione por data, para achar a execução de um dia sem procurar num arquivo único gigante.
3. Como desenvolvedor, quero no máximo 7 arquivos diários, para o `logs/` não crescer sem limite.
4. Como desenvolvedor, quero o terminal intacto, para continuar vendo o trace ao vivo enquanto o arquivo é gravado.
5. Como desenvolvedor, quero saber qual dia e qual fuso cada arquivo cobre, para não confundir o recorte.
6. Como quem roda os testes, quero apontar o diretório de log por variável de ambiente, para não escrever no `logs/` real nem depender de estado anterior.
7. Como quem mantém o demo, quero provar por teste que o arquivo existe e contém os spans de cada modo, para a feature não regredir em silêncio.
8. Como quem lê o arquivo, quero que ele não tenha códigos ANSI, para ser legível em `less`, `cat` e `grep`.
9. Como desenvolvedor que mede latência, quero que a escrita não bloqueie o thread do pipeline, para o `time.busy` continuar medindo trabalho e não disco.
10. Como desenvolvedor, quero que as últimas linhas não se percam no encerramento normal, para o arquivo ser confiável.
11. Como novo integrante do time, quero o comando de execução e o lugar do arquivo documentados no README, para entender o padrão em minutos.
12. Como tech lead, quero a decisão de fuso registrada, para ninguém esperar um recorte local que o arquivo não tem.
13. Como mantenedor, quero a versão do `tracing-appender` fixada, para builds reproduzíveis.
14. Como revisor, quero o escopo explícito (sem JSON, sem compressão, sem coletor), para não haver dúvida sobre o que o demo entrega.

## Decisões de implementação

- **Dependência:** `tracing-appender = "=0.2.5"` (research 0002), sem features. MSRV 1.63, compatível com `tracing 0.1.44` e `tracing-subscriber 0.3.23`. O pin segue o estilo do workspace.
- **Dois sinks, um trace:** um `registry()` com dois `fmt::Layer` — stdout com o formato atual (ANSI ligado), arquivo com `with_ansi(false)`. Ambos com `FmtSpan::CLOSE` e nível `INFO`. A composição do subscriber permanece no entrypoint do binário; o crate `demo` não ganha API pública nova.
- **Fuso — decisão consciente:** a crate só anexa data **em UTC** e o builder não oferece fuso nem formato. A descoberta pedia data local; a research mostrou que atendê-la exigiria rotação própria. **Decisão: aceitar UTC e documentar**, em vez de trocar o `RollingFileAppender` por uma rotação artesanal. Em UTC-3, o arquivo "do dia N" começa às 21:00 do dia N-1 local — o README diz isso.
- **Diretório e nome:** padrão `logs/` relativo ao diretório de execução (no `.gitignore`), sobrescrito por `DEMO_LOG_DIR`. Prefixo `demo.log` → `demo.log.yyyy-MM-dd`.
- **Retenção:** `max_log_files(7)`. É teto por contagem de arquivos, não por tamanho; a doc ressalva que a contagem pode ocasionalmente ficar abaixo do máximo, e que `0` desabilita a remoção. O README registra a ressalva.
- **Writer não bloqueante:** `tracing_appender::non_blocking` com o `WorkerGuard` ligado a uma variável nomeada no `main`, vivo até o fim. A fila é lossy por padrão (buffer de 128 mil linhas); o README registra que, sob saturação, linhas podem ser descartadas.
- **Flush no caminho de erro:** o `main` atual chama `std::process::exit(1)` no erro do pipeline, **depois** de o trace ter sido emitido; a doc do guard diz que `process::exit` pode perder as últimas linhas. O caminho de erro passa a **retornar `ExitCode`** (ou a dropar o guard antes de sair). O `exit(2)` de uso inválido permanece: acontece antes de instalar o subscriber.
- **Sem mudança no pipeline:** `handle`/`handle_aop`, os clientes de IO e a macro `#[aop_span]` não mudam. A feature é de destino do trace, não de instrumentação.
- **Tracker local:** o PRD vive em `docs/ultron/tasks/0002-log-rotativo-por-data/prd.md`; no tracker local, o `task.md` é a issue. O rótulo `ready-for-agent` não se aplica a um tracker sem rótulos.

## Decisões de teste

- **Testar comportamento externo, não o subscriber:** os testes observam o arquivo e o `stdout`, nunca o estado interno das layers.
- **Costura principal — o binário:** é a costura mais alta e a que o usuário vê. O padrão já existe em `demo/tests/cli.rs` e `demo/tests/aop_mode.rs`: executar o binário e afirmar sobre a saída. O teste do arquivo segue o mesmo desenho, com `DEMO_LOG_DIR` apontando para um diretório temporário.
- **Leitura depois do processo:** o teste lê o arquivo **depois** que o processo termina — é quando o guard dropa e o buffer esvazia. Ler com o processo vivo seria corrida.
- **Cenários, por modo:** um teste para `baseline` e um para `aop`, cada um afirmando: arquivo com a data UTC do dia; spans esperados presentes e os proibidos ausentes; `stdout` ainda com o trace; arquivo sem ANSI.
- **Retenção:** pré-semear 8 arquivos diários com o prefixo no diretório temporário e executar o binário; afirmar que o mais antigo sumiu e que sobraram no máximo 7. O teste verifica a nossa configuração, não a implementação da crate.
- **Data esperada no teste:** derivar a data UTC do dia em tempo de execução (dependência de desenvolvimento `time`, ou helper próprio), nunca fixar a data no teste.
- **Caminho de erro:** não é alcançável pela CLI hoje (o pedido do binário é fixo). A troca de `process::exit` por `ExitCode` fica como decisão de implementação; os testes de unidade existentes continuam cobrindo a propagação de erro do pipeline.
- **Prior art:** os testes de CLI e de modo; a captura própria de spans em `span_tree` e `aop_macro` **não muda** — eles instalam subscriber próprio e não passam pelo `main`.

## Avaliação de Dependências (SCA)

**Lacuna registrada, não um verde.** O `deps_audit.py` do arsenal não reconhece `Cargo.lock` (suporta npm, NuGet e locks Python), e `cargo audit` / `cargo deny` não estão instalados neste host. Portanto **não houve auditoria de CVE** para o workspace Rust nesta data.

Retrato atual (informativo, sem juízo de segurança): dependências diretas fixadas — `aop-macros` (local), `tokio =1.54.1`, `tracing =0.1.44`, `tracing-subscriber =0.3.23`, `syn =3.0.10`, `quote =1.0.47`, `proc-macro2 =1.0.107`. `Cargo.lock` com 230 linhas. A feature acrescenta `tracing-appender =0.2.5` e suas transitivas (`crossbeam-channel`, `time`, `thiserror`, `symlink`), que entram no lockfile depois do coder.

**Recomendação:** instalar `cargo-audit` e rodar `cargo audit` após adicionar a dependência, ou estender o `deps_audit.py` para Cargo. Enquanto isso, esta seção registra a ausência em vez de afirmar conformidade que ninguém verificou.

## Fora de escopo

- Formato JSON/estruturado, compressão dos arquivos antigos, envio para coletor (o OTLP continua documentado como caminho de produção, não implementado).
- Rotação por tamanho, configuração de nível por sink e formatter próprio.
- Rotação com data local — descartada conscientemente nesta versão; fica registrada como alternativa disponível caso o UTC documentado não baste.
- Alterar o pipeline, os clientes simulados ou a macro `#[aop_span]`.
- Benchmark do custo de logging.

## Notas adicionais

- A evidência externa está em `research.md` (research 0002): versão e MSRV, UTC fixo, `max_log_files`, semântica do `WorkerGuard` e composição das layers.
- O `verify_all.sh` do arsenal não reconhece a stack Rust (limitação já registrada na task 0001). A verificação deste projeto é manual: `cargo build`, `cargo clippy --workspace --all-targets -- -D warnings` e `cargo test`.
- O `logs/` nunca é versionado; o `.gitignore` da raiz passa a ignorá-lo.
- A demo continua rodando com um comando, sem variável de ambiente obrigatória; `DEMO_LOG_DIR` é uma conveniência para testes e para quem quiser outro destino.

## Critérios de aceitação

- [AC1] Executado com `DEMO_LOG_DIR` apontando para um diretório, o binário cria, no modo `baseline`, um arquivo cujo nome tem o prefixo `demo.log.` e a data UTC do dia, nesse diretório.
- [AC2] O arquivo do modo `baseline` contém os spans `handle`, `load_customer` e `persist` e **não** contém `validate`, `price`, `apply_rules` nem `render`.
- [AC3] O arquivo do modo `aop` contém os spans `validate`, `load_customer`, `price`, `apply_rules`, `persist` e `render` sob `handle`, com `time.busy` e `time.idle`.
- [AC4] O terminal continua recebendo o trace nos dois modos, no formato atual.
- [AC5] O arquivo não contém sequências de escape ANSI.
- [AC6] Com 8 arquivos diários pré-existentes no diretório apontado, uma execução deixa no máximo 7 arquivos. A poda é por ordem de criação (btime), com fallback para a data do nome quando o sistema não informa btime; com um arquivo por dia, é o mais antigo que sai.
- [AC7] Sem `DEMO_LOG_DIR`, os arquivos vão para `./logs` sob o diretório de execução, e `logs/` está no `.gitignore`.
- [AC8] O README documenta o diretório, a rotação diária em UTC (com o deslocamento local), a retenção de 7, `DEMO_LOG_DIR`, o comportamento não bloqueante com guard e o comando de execução.
- [AC9] O `Cargo.toml` fixa `tracing-appender = "=0.2.5"`; build, clippy e a suíte completa — incluindo os 9 testes existentes — seguem verdes.

---

## Proveniência

Descoberta da task 0002 (2026-11-18) e `research.md` da mesma task — versão e MSRV do `tracing-appender`, UTC fixo da rotação, retenção nativa, semântica do `WorkerGuard` e composição de dois `fmt` layers.

## Histórico de versões

| Versão | Data | Autor | Mudança |
|---|---|---|---|
| 1.2 | 2026-11-18 | ULTRON · ultron-review | AC7 e decisão de diretório alinhadas ao comportamento: `./logs` relativo ao diretório de execução |
| 1.1 | 2026-11-18 | ULTRON · ultron-tdd | AC6 ajustado ao contrato testável: teto de 7; a crate poda por btime, com fallback para a data do nome |
| 1.0 | 2026-11-18 | ULTRON · ultron-prd | Versão inicial, a partir da descoberta e da research 0002 |

## Rastreabilidade

Task 0002-log-rotativo-por-data › PRD-0002

<sub>Gerado por ULTRON v2.1.1 · skill ultron-prd · 2026-11-18T15:55:00-03:00 · em conformidade com a Convenção de Documento ULTRON. Documento versionado em git — a proveniência viaja com o projeto.</sub>
