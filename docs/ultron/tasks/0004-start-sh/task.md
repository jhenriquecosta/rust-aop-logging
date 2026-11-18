# Task 0004-start-sh — Versiona o start.sh e documenta no README

tipo: chore
branch: chore/0004-start-sh
publicacao: pendente
estado: concluído
worktree: .claude/worktrees/chore-0004-start-sh
responsavel: jhenriquecosta@hotmail.com
next_skill: 

Adicionar ao repositório o start.sh gerado pelo ultron-preview: executável, com o bloco-manifesto e uso `./start.sh [baseline|aop]` (padrão aop). Documentar na seção 'Como rodar' do README. O script faz cd para o diretório dele e delega ao cargo run.

## Critérios de aceitação

- [x] `start.sh` versionado, executável, com uso `./start.sh [baseline|aop]` (padrão `aop`).
- [x] README documenta o script na seção "Como rodar".
- [x] `./start.sh aop` e `./start.sh baseline` executam; suíte 16/16 e clippy limpos.
