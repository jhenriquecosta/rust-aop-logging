# Revisão — task 0002-log-rotativo-por-data

**Data:** 2026-11-18 · **Branch:** feature/0002-log-rotativo-por-data · **Base:** develop
**Perfil de modelo:** review — sem `models.yaml` (a oferta de política de modelos foi recusada pelo Operador em 2026-11-18)
**Revisor distinto do codificador:** sim para o modelo desta sessão — o parecer foi emitido por subagente em modelo Opus, distinto do modelo da sessão; o modelo da sessão que implementou não ficou registrado nos artefatos, então a distinção plena não pode ser provada.

---

## Veredito

- [ ] ✅ Conforme — sem achados críticos ou médios
- [x] 🟡 Conforme com ressalvas — 5 achado(s) médio(s), todos corrigidos antes do fechamento (commit `bd321f0`)
- [ ] 🔴 Bloqueado — 0 achado(s) crítico(s)

---

## Achados

*Regra: todo achado cita o artefato que sustenta. Sem artefato, não é achado.*

### 🔴 Críticos

Nenhum.

### 🟡 Médios

| # | Achado | Artefato violado | Arquivo/linha | Correção |
|---|--------|------------------|---------------|----------|
| M1 | O teste de retenção afirmava o teto de 7 e a existência do arquivo do dia, mas não provava qual arquivo saía; o comentário reconhecia a lacuna. | `slices/02-retencao.md` · `prd.md` AC6 | `demo/tests/file_log.rs:243-284` | Semeia com 300 ms entre criações (btime distinto e ordenado) e afirma que `demo.log.2026-11-01` foi removido. |
| M2 | A ausência de ANSI só era testada no `baseline`; o arquivo do `aop` foi verificado apenas à mão. | `slices/01-arquivo-diario.md:12` · `prd.md:57` | `demo/tests/file_log.rs:92-109` | `arquivo_nao_tem_codigos_ansi_nos_dois_modos` percorre `baseline` e `aop`. |
| M3 | O README não registrava a fila lossy e o descarte por saturação, como o PRD decide. | `prd.md:47` | `README.md:80-107` | Parágrafo "Encerramento e saturação": buffer padrão de 128 mil linhas; sob saturação, linhas podem ser descartadas. |
| M4 | O README não declarava o escopo "sem JSON/estruturado, sem compressão" pedido pela fatia 3. | `slices/03-readme.md:13` | `README.md:137-141` | Bullet "Formato texto, sem compressão" em Decisões e limitações. |
| M5 | O PRD dizia "`logs/` na raiz do projeto", mas código, teste e README usam o diretório corrente. | `prd.md:45` · `prd.md` AC7 | `demo/src/main.rs:51-53` · `README.md:84,88` | PRD v1.2: decisão e AC7 passam a dizer "`./logs` relativo ao diretório de execução", como a fatia 1 e o README. |

---

## Dimensões conferidas

| # | Dimensão | Status | Nota |
|---|----------|--------|------|
| 1 | Escopo (PRD/slices) | ✓ | As três fatias entregues; pipeline, clientes e macro intactos (`git diff` vazio em `demo/src/lib.rs` e `aop-macros/`). |
| 2 | Arquitetura (ADRs) | — sem artefato | Sem `docs/ultron/agents/adr/`; as decisões do PRD foram conferidas (M5 corrigido). |
| 3 | Risco de domínio (checklist) | — não aplicável | A task não declara checklist. |
| 4 | Observabilidade (logging.md) | — sem artefato | Sem `logging.md`; nenhum segredo/PII: `id=1` é simulado e `persist` faz `skip` do pedido. |
| 5 | Escopo em legado (refactor_policy) | — não é legado | Repositório novo. |
| 6 | Critérios de aceitação | ✓ | AC1–AC9 atendidos, com evidência de teste e execução. |
| 7 | A saída, não só o cálculo | ✓ | Testes sobre o terminal e o arquivo; retenção e ANSI cobertos após as correções. |

---

## Critérios de aceitação — evidência

| AC | Status | Evidência |
|---|---|---|
| AC1 | **ATENDIDO** | `baseline_com_demo_log_dir_cria_o_arquivo_do_dia`; execução manual com `DEMO_LOG_DIR` criou `demo.log.2026-11-18`. |
| AC2 | **ATENDIDO** | `baseline_arquivo_tem_spans_de_io_e_nao_de_cpu`; arquivo real com `handle`, `load_customer`, `persist`, sem CPU. |
| AC3 | **ATENDIDO** | `aop_arquivo_tem_um_span_por_etapa_sob_handle`; seis etapas sob `handle`, com `time.busy`/`time.idle`. |
| AC4 | **ATENDIDO** | `terminal_continua_com_o_trace_nos_dois_modos`; stdout com ANSI e `resposta:`. |
| AC5 | **ATENDIDO** | `arquivo_nao_tem_codigos_ansi_nos_dois_modos`; verificação manual com `grep` = 0 ESC nos dois modos. |
| AC6 | **ATENDIDO** | `retencao_mantem_no_maximo_sete_arquivos_diarios`; execução manual com btimes escalonados removeu os dois mais antigos. |
| AC7 | **ATENDIDO** | `.gitignore` com `logs/`; `sem_demo_log_dir_usa_logs_sob_o_cwd` prova `<cwd>/logs`; PRD v1.2 alinhado. |
| AC8 | **ATENDIDO** | `README.md:84-107` e `:178-180`: diretório, `DEMO_LOG_DIR`, rotação UTC com deslocamento, retenção, guard, saturação, escopo e comandos. |
| AC9 | **ATENDIDO** | `tracing-appender = "=0.2.5"` em `demo/Cargo.toml:11`; build, clippy e **16 testes** verdes. |

---

## Critérios por fatia

| Fatia | Status | Nota |
|---|---|---|
| 01-arquivo-diario | Atendida | Dependência, dois layers, appender, guard, erro por guard/exit, `.gitignore`, testes por modo; ANSI agora nos dois modos. |
| 02-retencao | Atendida | `max_log_files(7)`; teste prova o teto e a remoção do mais antigo; README com a ressalva. |
| 03-readme | Atendida | Diretório, nome, UTC com deslocamento, retenção, guard, saturação, JSON/compressão e comandos. |

---

## Observações do revisor (não são achados)

- **Cor do stdout:** com o layer do arquivo primeiro, os campos de span no terminal perdem a cor; timestamp, nível, nome do span e `time.*` continuam coloridos. A `research.md` previa a ordem do arquivo primeiro como contorno; a exceção foi registrada em `task.md`.
- **SCA:** `cargo audit`/`cargo deny` não estão instalados e o `deps_audit.py` não lê `Cargo.lock`; não houve auditoria de CVE. O PRD registra a lacuna e recomenda instalar o `cargo-audit`; a exceção foi registrada em `task.md`.
- **`docs/ultron/agents/stack.md`** continua `stack: n/d` — fora do escopo desta task, registrado no `tdd-progress.md`.
- **"Panic não capturado"** no README era impreciso (o flush ocorre durante o unwinding); o texto passou a citar `std::process::exit`/abort.

---

## Verificação executada

| Comando | Resultado |
|---|---|
| `cargo test` | 16 passaram, 0 falharam (9 existentes + 7 novos) |
| `cargo clippy --workspace --all-targets -- -D warnings` | limpo |
| `cargo build` | OK |
| `cargo run --quiet -- baseline` / `aop` com `DEMO_LOG_DIR` | arquivos criados; conteúdo por modo conferido |
| Sem `DEMO_LOG_DIR`, a partir de um cwd temporário | `<cwd>/logs/demo.log.2026-11-18` criado |
| `DEMO_LOG_DIR` inexistente (subdiretórios) | diretório criado; arquivo gerado |
| Modo inválido | exit 2, sem log criado |
| Retenção manual com 8 arquivos e btimes distintos | 7 restantes; os dois mais antigos removidos |
| `.gitignore` | `git check-ignore` confirma `logs/` |

**Limites desta revisão:** a distinção UTC vs. local não é observável hoje (as datas coincidem em UTC-3 no momento); a evidência vem da documentação da crate, do teste que deriva a data UTC e do nome gerado. O fallback de retenção sem btime foi confirmado no fonte da crate, não em execução. Perda de linhas por abort não foi exercitada.

---

## Exceções registradas

Nenhum achado crítico escapado. O trade-off de ANSI e a lacuna de SCA estão registrados na seção "Exceções registradas" da task.

---

*Parecer de conformidade emitido por subagente independente em 2026-11-18, consolidado pelo agente da sessão. Os cinco achados médios foram corrigidos no commit `bd321f0`. Nenhum arquivo do projeto foi alterado durante o parecer.*
