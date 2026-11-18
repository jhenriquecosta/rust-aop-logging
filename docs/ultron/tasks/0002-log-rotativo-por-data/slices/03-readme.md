# Fatia 3 — README e fuso documentado

## O que construir

Seção nova no README, curta e verificável contra o comportamento real:

- o diretório padrão `logs/` e a sobrescrita por `DEMO_LOG_DIR`;
- o nome `demo.log.yyyy-MM-dd`;
- a rotação **diária em UTC**, com o deslocamento local explícito (em UTC-3, o arquivo "do dia N" começa às 21:00 do dia N-1 local);
- a retenção de 7 e a ressalva de contagem (teto, pode ficar abaixo);
- o writer não bloqueante com guard e o risco de perder as últimas linhas num encerramento abrupto;
- o comando de execução e onde o arquivo aparece;
- o escopo explícito: sem JSON/estruturado, sem compressão, sem coletor (OTLP continua só documentado), sem rotação por tamanho.

## Critérios de aceite

- [x] AC8 — o README documenta diretório, rotação diária em UTC (com o deslocamento local), retenção de 7, `DEMO_LOG_DIR`, o comportamento não bloqueante com guard e o comando de execução.

## Bloqueada por

Fatias 1 e 2.
