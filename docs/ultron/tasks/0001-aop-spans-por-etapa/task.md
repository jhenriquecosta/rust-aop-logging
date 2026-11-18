# Task 0001-aop-spans-por-etapa — Demonstração de AOP em Rust: spans por etapa de processamento

tipo: feature
branch: feature/0001-aop-spans-por-etapa
estado: em andamento
worktree: .claude/worktrees/feature-0001-aop-spans-por-etapa
responsavel: jhenriquecosta@hotmail.com
next_skill: ultron-discover

## Objetivo

Criar um projeto de demonstração do uso de AOP (programação orientada a aspectos) em Rust para instrumentação de logging/telemetria, motivado por um problema real de latência: em produção, vários blocos de processamento aparecem unidos em um único span, e não é possível saber quanto tempo cada passo levou. Como esses blocos não fazem IO, não são instrumentados automaticamente.

## O que a demonstração precisa mostrar

- Separação de spans por etapa de processamento, inclusive para blocos CPU-bound sem IO.
- Duração por passo registrada e visível (tracing/OpenTelemetry).
- A parte AOP: a instrumentação aplicada de forma declarativa (ex.: attribute macro procedural, #[instrument] ou macro própria), sem espalhar código de telemetria pela lógica de negócio.
- Comparação entre o que é instrumentado automaticamente (IO/middleware) e o que exige instrumentação explícita.

## Contexto de produção (motivador)

Blocos unidos em um único span; sem IO, não há instrumentação automática; o objetivo é medir o tempo de cada passo do processamento.

## Fora de escopo (a confirmar na descoberta)

- Aplicar a mudança no sistema de produção (outro repositório).
- Trocar a stack de observabilidade do time.

## Critérios de aceitação (rascunho — a refinar na descoberta)

- [ ] O projeto compila e roda com cargo run.
- [ ] Há um exemplo com etapas de processamento que não fazem IO, cada uma em seu próprio span.
- [ ] A saída mostra a duração de cada etapa separadamente.
- [ ] A instrumentação é aplicada por um mecanismo de aspecto (macro/atributo), não por chamadas manuais espalhadas.
