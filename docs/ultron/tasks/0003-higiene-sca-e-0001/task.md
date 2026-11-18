# Task 0003-higiene-sca-e-0001 — Higiene: auditoria de dependências, stack.md e médios da revisão da 0001

tipo: chore
branch: chore/0003-higiene-sca-e-0001
publicacao: pendente
estado: concluído
worktree: .claude/worktrees/chore-0003-higiene-sca-e-0001
responsavel: jhenriquecosta@hotmail.com
next_skill: 

Fechar as pendências registradas nas tasks recentes.

- SCA: instalar o `cargo-audit` (mudança de ambiente — aprovação do Operador), rodar sobre o `Cargo.lock` e registrar o snapshot das dependências. O `deps_audit.py` do arsenal não lê Cargo.lock; registrar a lacuna e o método usado.
- `docs/ultron/agents/stack.md`: trocar `n/d` pela stack real (Rust 1.85+ com cargo; tracing 0.1.44, tracing-subscriber 0.3.23, tracing-appender 0.2.5, tokio 1.54.1, syn 3.0.10; origem: novo).
- Médios da revisão da 0001 (review.md): M1 — o teste de `name` é vacuamente verdadeiro (o nome da função é igual ao do span); corrigir com nome distinto. M2 — o PRD-0001 diz CPU com iterações ajustáveis; implementar ou corrigir o texto. M3 — a camada de captura não registra entrada/fechamento apesar do PRD; implementar os callbacks ou corrigir o texto. M5 — o PRD-0001 diz que a macro emite evento de conclusão; corrigir o texto, quem emite é o FmtSpan::CLOSE. M4 (SCA) coberto no primeiro item.

## Critérios de aceitação

- [x] SCA — `cargo-audit` instalado e executado; snapshot em `deps-audit.md` (0 vulnerabilidades em 40 dependências).
- [x] `stack.md` — stack Rust real, sem `n/d`.
- [x] M1 — o teste de `name` prova a renomeação com uma função de nome distinto.
- [x] M2 — PRD-0001 corrigido: CPU determinística, sem iterações ajustáveis.
- [x] M3 — as camadas de captura registram entrada e fechamento; o teste do `aop` afirma o ciclo de vida.
- [x] M5 — PRD-0001 corrigido: o evento de close vem do `FmtSpan::CLOSE`.

## Exceções registradas

- Portão `verify` do arsenal (2026-11-18): o `verify_all.sh` não tem ramo Rust e sai `2` (nada verificado). A verificação foi manual: `cargo build`, `cargo clippy --workspace --all-targets -- -D warnings` e `cargo test` — 16 testes verdes. O SCA foi feito com `cargo audit` (ver `deps-audit.md`), porque o `deps_audit.py` não lê `Cargo.lock`.
