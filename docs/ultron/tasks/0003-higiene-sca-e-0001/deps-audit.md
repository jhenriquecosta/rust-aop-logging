# Auditoria de dependências — task 0003

**Data:** 2026-11-18
**Ferramenta:** `cargo-audit 0.23.0`, instalado nesta data via `cargo install cargo-audit --locked`
**Base de avisos:** RustSec advisory-db, 1.533 avisos carregados
**Comando:** `cargo audit` no workspace, sobre o `Cargo.lock`
**Resultado:** **0 vulnerabilidades**; exit 0. Nenhum aviso de crate não mantida ou banida foi reportado.

## Cobertura

Dependências diretas: `aop-macros` (local), `tokio 1.54.1`, `tracing 0.1.44`, `tracing-subscriber 0.3.23`, `tracing-appender 0.2.5`, `syn 3.0.10`, `quote 1.0.47`, `proc-macro2 1.0.107`. O lockfile resolve **40 crates** no total, e a auditoria cobre todas.

## Lacuna do arsenal

O `deps_audit.py` do `ultron-deps-audit` não lê `Cargo.lock` — suporta npm, NuGet e locks Python. Para Rust, a rota usada é o `cargo audit`, que consulta o RustSec. Recomendação: estender o `deps_audit.py` para Cargo, ou documentar o `cargo audit` como rota oficial de SCA para Rust no arsenal.

## Limite deste snapshot

É um retrato da data: a base RustSec muda, e uma vulnerabilidade publicada depois desta execução não aparece aqui. Reexecutar `cargo audit` periodicamente — e antes de uma release.
