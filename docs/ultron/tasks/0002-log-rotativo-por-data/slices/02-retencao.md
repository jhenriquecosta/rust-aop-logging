# Fatia 2 — Retenção de 7

## O que construir

- `.max_log_files(7)` no builder do appender, como teto por contagem de arquivos diários.
- Teste de retenção: pré-semear 8 arquivos `demo.log.<data>` no diretório temporário `DEMO_LOG_DIR`, executar o binário e afirmar que o mais antigo foi removido e que restaram no máximo 7. O teste verifica a nossa configuração, não a implementação da crate.
- Registrar no README (fatia 3) a ressalva da crate: a contagem retida pode ocasionalmente ficar abaixo do máximo; 7 é teto, não piso.

## Critérios de aceite

- [ ] AC6 — com 8 arquivos diários pré-existentes, uma execução deixa no máximo 7 e remove o mais antigo.

## Bloqueada por

Fatia 1.
