# Task 0005-symlinks-ultron — Desversiona os symlinks do .claude/skills e documenta a instalação

tipo: chore
branch: chore/0005-symlinks-ultron
publicacao: pendente
estado: concluído
worktree: .claude/worktrees/chore-0005-symlinks-ultron
responsavel: jhenriquecosta@hotmail.com
next_skill: 

Remover do índice os symlinks de .claude/skills — eles apontam para o checkout local do arsenal (/home/chefe/workspaces/CoE/ultron-skills) e quebram no GitHub e em qualquer clone. Ignorar .claude/skills/ no git e documentar no README que as skills do Ultron são instaladas localmente, com os artefatos do processo em docs/ultron/. Os symlinks no disco não são alterados.

## Critérios de aceitação

- [x] Os 74 symlinks de `.claude/skills` saíram do índice (`git ls-files .claude/skills` = 0) e continuam no disco.
- [x] `.claude/skills/` está no `.gitignore` (`git check-ignore` confirma).
- [x] O README documenta que as skills são instaladas localmente e não versionadas.
