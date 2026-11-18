# Task 0003-higiene-sca-e-0001 — Higiene: auditoria de dependências, stack.md e médios da revisão da 0001

tipo: chore
branch: chore/0003-higiene-sca-e-0001
estado: em andamento
worktree: .claude/worktrees/chore-0003-higiene-sca-e-0001
responsavel: jhenriquecosta@hotmail.com
next_skill: ultron-coder

Fechar as pendências registradas nas tasks recentes.

- SCA: instalar o `cargo-audit` (mudança de ambiente — aprovação do Operador), rodar sobre o `Cargo.lock` e registrar o snapshot das dependências. O `deps_audit.py` do arsenal não lê Cargo.lock; registrar a lacuna e o método usado.
- `docs/ultron/agents/stack.md`: trocar `n/d` pela stack real (Rust 1.85+ com cargo; tracing 0.1.44, tracing-subscriber 0.3.23, tracing-appender 0.2.5, tokio 1.54.1, syn 3.0.10; origem: novo).
- Médios da revisão da 0001 (review.md): M1 — o teste de `name` é vacuamente verdadeiro (o nome da função é igual ao do span); corrigir com nome distinto. M2 — o PRD-0001 diz CPU com iterações ajustáveis; implementar ou corrigir o texto. M3 — a camada de captura não registra entrada/fechamento apesar do PRD; implementar os callbacks ou corrigir o texto. M5 — o PRD-0001 diz que a macro emite evento de conclusão; corrigir o texto, quem emite é o FmtSpan::CLOSE. M4 (SCA) coberto no primeiro item.
