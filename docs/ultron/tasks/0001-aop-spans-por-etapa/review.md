# Revisão — task 0001-aop-spans-por-etapa

**Data:** 2026-11-18 · **Branch:** feature/0001-aop-spans-por-etapa · **Base:** develop
**Perfil de modelo:** review — sem `models.yaml` (a oferta de política de modelos foi recusada pelo Operador em 2026-11-18)
**Revisor distinto do codificador:** sim para o modelo desta sessão — o parecer foi emitido por subagente em modelo Opus, distinto do modelo da sessão; o modelo da sessão que implementou (commits de 2026-11-18) não ficou registrado nos artefatos, então a distinção plena não pode ser provada.

---

## Veredito

- [ ] ✅ Conforme — sem achados críticos ou médios
- [x] 🟡 Conforme com ressalvas — 5 achado(s) médio(s) registrado(s)
- [ ] 🔴 Bloqueado — 0 achado(s) crítico(s)

---

## Achados

*Regra: todo achado cita o artefato que sustenta. Sem artefato, não é achado.*

### 🔴 Críticos

Nenhum.

### 🟡 Médios

| # | Achado | Artefato violado | Arquivo/linha |
|---|--------|------------------|---------------|
| M1 | O teste de `#[aop_span(name = "...")]` é vacuamente verdadeiro: a função se chama `etapa_renomeada` e o atributo usa `name = "etapa_renomeada"` — igual ao nome padrão. A asserção positiva passaria mesmo se a macro ignorasse `name`; a negativa procura `etapa_renomeada_fn`, nome que o código nunca produziria. A implementação está correta (`aop-macros/src/lib.rs:32-36,41`), mas o critério não tem prova válida. **Corrigir:** dar à função um nome diferente do span e afirmar que o span renomeado existe e o nome original não. | `slices/02-macro-aop.md:18` | `demo/tests/aop_macro.rs:28-49` |
| M2 | A CPU simulada não tem "contagem de iterações ajustável": as impls são aritmética trivial sobre os dois itens do pedido, sem constante, parâmetro ou variável de ambiente que ajuste o esforço. As durações observadas são mensuráveis (µs), mas o mecanismo registrado no PRD não existe. **Corrigir:** adicionar a contagem ajustável às impls, ou corrigir a redação do PRD para o que foi entregue. | `prd.md:52` | `demo/src/lib.rs:105-140` |
| M3 | A camada de captura dos testes registra apenas criação (`on_new_span`) e, no `span_tree`, o pai. Não há `on_enter`/`on_exit`/`on_close`, então o ciclo de vida dos spans declarado no PRD não é observado por teste algum. AC5 (presença/ausência) continua provado. **Corrigir:** implementar os callbacks de entrada/fechamento e afirmar o ciclo, ou registrar o desvio consciente. | `prd.md:62` · `prd.md:68` | `demo/tests/span_tree.rs:39-49` · `demo/tests/aop_macro.rs:19-26` |
| M4 | O snapshot do `/ultron-deps-audit` não foi registrado após o primeiro build, como o PRD pede. O `Cargo.lock` existe e não contém dependência proibida, mas não há artefato de auditoria em `docs/ultron/`. Não é AC e não afeta comportamento. **Corrigir:** rodar o `/ultron-deps-audit` e registrar o snapshot, ou registrar o escape consciente. | `prd.md:74` | `Cargo.lock` (sem artefato de auditoria correspondente) |
| M5 | O PRD diz que a macro "emite um evento de conclusão com o tempo decorrido", mas a macro não emite evento algum: ela cria o span e o `FmtSpan::CLOSE` do subscriber sintetiza o evento de fechamento, que é o que mostra `time.busy`/`time.idle`. O comportamento observável (AC3) está atendido; a redação da decisão é que diverge do código. **Corrigir:** ajustar o texto do PRD (o desenho atual é o suportado pela research Q2), ou implementar a emissão do evento na macro. | `prd.md:53` | `aop-macros/src/lib.rs:29-54` |

---

## Dimensões conferidas

| # | Dimensão | Status | Nota |
|---|----------|--------|------|
| 1 | Escopo (PRD/slices) | ✓ | Sem HTTP, OTLP implementado, `aspect-rs` como dependência ou benchmark — o "Fora de escopo" do PRD é respeitado. Tudo o que foi entregue está nos quatro slices. |
| 2 | Arquitetura (ADRs) | — sem artefato | O repositório não tem `docs/ultron/agents/adr/`. As decisões do PRD foram conferidas; M2 e M5 são divergências de redação/implementação registradas. |
| 3 | Risco de domínio (checklist) | — não aplicável | A task não declara checklist de risco. |
| 4 | Observabilidade (logging.md) | — sem artefato | Não existe `docs/ultron/agents/logging.md`; não há convenção escrita a conferir. Nenhum segredo/PII nos logs: `id=1` é simulado e `persist` faz `skip` do pedido. |
| 5 | Escopo em legado (refactor_policy) | — não é legado | Repositório novo (`origem: novo`), sem baseline de legado. |
| 6 | Critérios de aceitação | ✓ | AC1–AC8 atendidos, com evidência de execução e de teste (tabela abaixo). |
| 7 | A saída, não só o cálculo | ✓ com ressalva | Há testes sobre a saída real (`cli`, `aop_mode`). A duração no modo `aop` é verificada por execução, não por asserção automatizada; o teste de renomeação é vácuo (M1). |

---

## Critérios de aceitação — evidência

| AC | Status | Evidência |
|---|---|---|
| AC1 — baseline com span externo + IO, sem CPU | **ATENDIDO** | Execução: `handle:load_customer{id=1}`, `handle:persist`, `handle`; nenhum span CPU. Testes: `demo/tests/cli.rs:17-26`, `demo/tests/span_tree.rs:61-82`. |
| AC2 — aop com um span por etapa | **ATENDIDO** | Execução: `validate`, `load_customer`, `price`, `apply_rules`, `persist`, `render` sob `handle`. Testes: `demo/tests/aop_mode.rs:3-29`, `demo/tests/span_tree.rs:84-111`. |
| AC3 — duração (busy/idle) nos dois modos | **ATENDIDO** | Execução nos dois modos com timestamp e `time.busy`/`time.idle` em todas as linhas de close. Asserção automatizada só no baseline (`demo/tests/cli.rs:27-30`). |
| AC4 — CPU via `#[aop_span]`, sem span manual nos corpos | **ATENDIDO** | `demo/src/lib.rs:163-182`; os corpos puros não têm `span!`/`info_span!`. |
| AC5 — captura prova presença no aop e ausência no baseline | **ATENDIDO** | `demo/tests/span_tree.rs:61-111`, com `Layer` própria. |
| AC6 — macro em `fn` e `async fn`, retorno e erro | **ATENDIDO** | `demo/tests/aop_macro.rs:52-86`. |
| AC7 — README (AOP, antes/depois, aspect-rs, OTLP) | **ATENDIDO** | `README.md:7-12` · `:41-72` · `:92-100` · `:104-108`. |
| AC8 — versões fixadas, edição 2021, MSRV 1.85 | **ATENDIDO** | `aop-macros/Cargo.toml:4-13`, `demo/Cargo.toml:4-11`, `Cargo.lock`; `cargo build --locked` OK. |

---

## Critérios por slice

| Fatia | Critérios | Status |
|---|---|---|
| 01-baseline-pipeline | 4 critérios | Todos atendidos (execução + `cli.rs` + `span_tree.rs`). |
| 02-macro-aop | 5 critérios | 4 atendidos; o de `name = "..."` é atendido na implementação, mas não provado por teste (M1). |
| 03-macro-async | 3 critérios | Atendidos; a ressalva de não depender dos eventos do fmt vale para os testes de captura — os de saída leem o texto do formatter de propósito. |
| 04-readme | 3 critérios | Atendidos; instruções executadas com sucesso. |

---

## Tensões avaliadas e não tratadas como achado

- **`demo/tests/cli.rs:27-30` depende de `time.busy`**, texto produzido pelo evento sintetizado do `FmtSpan::CLOSE`. Lido ao pé da letra, conflita com `prd.md:68` ("os testes não podem depender dos eventos sintetizados"). Não é achado: a decisão tem como alvo a captura de spans (satisfeita por `span_tree`/`aop_macro`, com `Layer` própria), e o teste de CLI é justamente o que a dimensão 7 exige — verificar a saída que o usuário vê.
- **Trechos do README** (`README.md:52-72`) omitem timestamp e códigos ANSI e usam durações ilustrativas; formato, ordem, nomes de span e campos conferem com a execução real, e o texto avisa que as durações variam. Trecho representativo, não cópia verbatim.
- **Checkboxes das fatias 01–04 continuam `[ ]`** nos arquivos de `slices/`, embora `task.md:61-64` as declare concluídas e verificadas. Desatualização de bookkeeping, não violação de artefato.
- **`docs/ultron/tasks/task-log.md` não rastreado** no worktree, pré-existente e fora do diff; não faz parte desta task.

---

## Verificação executada

| Comando | Resultado |
|---|---|
| `cargo build --workspace` | OK |
| `cargo clippy --workspace --all-targets -- -D warnings` | limpo |
| `cargo test` | 9 passaram, 0 falharam |
| `cargo run --quiet -- baseline` | exit 0; `handle` + 2 spans de IO; sem spans CPU; `time.busy`/`time.idle` presentes |
| `cargo run --quiet -- aop` | exit 0; 6 spans de etapa sob `handle`; `time.busy`/`time.idle` presentes |
| `cargo build --locked` | OK; dependências diretas conferem com o PRD |
| `rustc --version` | 1.100.0 (confere com o README) |

---

## Exceções registradas

Nenhum achado crítico; nenhum escape de revisão.

---

*Parecer de conformidade emitido por subagente independente em 2026-11-18 e consolidado pelo agente da sessão. Nenhum arquivo do projeto foi alterado durante a revisão.*
