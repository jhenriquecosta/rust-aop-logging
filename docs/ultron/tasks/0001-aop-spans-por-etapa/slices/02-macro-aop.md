# Fatia 2 — Macro #[aop_span] e modo aop

## O que construir

Crate `aop-macros` (proc-macro) com a macro de atributo `#[aop_span]` e o modo `aop` do binário.

- `#[aop_span]` cria um span no nível `INFO` com o nome da função (ou `name = "..."`), aplicada a funções síncronas.
- Wrappers anotados para as etapas CPU: `validate`, `price`, `apply_rules`, `render`, cada um delegando ao corpo puro (`*_impl`).
- CLI: `cargo run -- aop` executa o mesmo pipeline chamando os wrappers anotados.
- O modo `baseline` continua chamando os corpos puros e não ganha spans de CPU.
- A lógica dos passos não é duplicada: só a camada de aspecto difere entre os modos.

## Critérios de aceite

- [ ] `cargo run -- aop` imprime um span por etapa: `validate`, `load_customer`, `price`, `apply_rules`, `persist`, `render`, sob o span `handle`.
- [ ] No modo `baseline`, nenhum span de etapa CPU aparece.
- [ ] As etapas CPU são instrumentadas por `#[aop_span]`; não há `span!`/`info_span!` manual nos corpos.
- [ ] `#[aop_span(name = "...")]` renomeia o span criado.
- [ ] O workspace continua compilando com as versões fixadas.

## Bloqueada por

Fatia 1 (pipeline e contrato de saída).
