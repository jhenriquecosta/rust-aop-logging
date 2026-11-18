# Fatia 1 — Arquivo diário no binário

## O que construir

O caminho vertical completo do trace em arquivo, sem tocar no pipeline.

- Dependência `tracing-appender = "=0.2.5"` no crate `demo` (sem features).
- Substituir o `tracing_subscriber::fmt()` global do binário por um `registry()` com **dois** `fmt::Layer`: o de stdout com o formato atual (ANSI ligado) e o de arquivo com `with_ansi(false)`; ambos com `with_span_events(FmtSpan::CLOSE)` e nível `INFO`.
- Appender diário: `RollingFileAppender` com rotação `DAILY` e prefixo `demo.log` → `demo.log.yyyy-MM-dd`, no diretório `DEMO_LOG_DIR` ou `./logs` quando a variável não existe; criar o diretório se faltar.
- Writer não bloqueante: `tracing_appender::non_blocking` com o `WorkerGuard` ligado a uma variável nomeada no `main`, viva até o fim. O caminho de erro do pipeline passa a retornar `ExitCode` em vez de `std::process::exit(1)`; o `exit(2)` de uso inválido permanece, porque acontece antes de instalar o subscriber.
- `logs/` no `.gitignore` da raiz.
- Testes de integração novos: executar o binário com `DEMO_LOG_DIR` apontando para um diretório temporário, por modo, e afirmar nome com a data UTC do dia, spans presentes/ausentes, `stdout` com o trace e arquivo sem ANSI; a leitura acontece depois que o processo termina. A data esperada é derivada em tempo de execução (dev-dependency `time` ou helper próprio).
- O pipeline, os clientes simulados e a macro `#[aop_span]` não mudam.

## Critérios de aceite

- [ ] AC1 — com `DEMO_LOG_DIR`, o modo `baseline` cria o arquivo `demo.log.<data UTC do dia>` no diretório apontado.
- [ ] AC2 — o arquivo do `baseline` tem `handle`, `load_customer` e `persist`, e não tem `validate`, `price`, `apply_rules` nem `render`.
- [ ] AC3 — o arquivo do `aop` tem as seis etapas sob `handle`, com `time.busy` e `time.idle`.
- [ ] AC4 — o terminal continua recebendo o trace nos dois modos.
- [ ] AC5 — o arquivo não contém escape ANSI.
- [ ] AC7 — sem `DEMO_LOG_DIR`, os arquivos vão para `logs/` na raiz; `logs/` está no `.gitignore`.
- [ ] AC9 — `tracing-appender = "=0.2.5"` fixado; build, clippy e os 9 testes existentes seguem verdes.

## Bloqueada por

Nenhuma — pode começar imediatamente.
