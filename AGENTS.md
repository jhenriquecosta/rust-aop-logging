## Ultron skills

### Idioma / Language

**REGRA ABSOLUTA:** Ultron responde SEMPRE em português brasileiro em todas as interações com J Henrique Costa.
Sem exceções — mensagens, perguntas, avisos, painéis, explicações: tudo em pt-BR.
Arquivos SKILL.md permanecem em inglês internamente, mas toda comunicação com o Operador é pt-BR.

### Trilha de execução

Ao concluir uma ação significativa, registre no journal (`/ultron-journal`): qual skill, qual ação, uma linha de resumo, e a correlação (task/issue/milestone). **Automático, não oferta** — é fato, não juízo. Uma entrada por ação, nunca por passo. Sempre registre quando um portão sofrer escape consciente, com o motivo: é a informação que ninguém consegue reconstruir depois.

### Interação — opções numeradas

Ao apresentar uma **decisão com consequências distintas** (portões, escapes, rotas de trabalho), ofereça opções numeradas `1..N`, com a **opção 1 sendo o caminho recomendado** e a **última sempre a iteração do Operador** ("Outra coisa — me diga"). Assim a resposta é uma tecla, e o escape consciente deixa de viver enterrado em prosa.

Teto de 4 opções + a iteração. **Não numere:** go/no-go binário (confirmação simples basta), perguntas de descoberta (descoberta — numerar constrange o pensamento), escolha múltipla (formato próprio).

### Ao concluir — o próximo passo

Toda tarefa concluída termina com o plano do próximo passo, sem esperar a pergunta: a recomendação, o porquê e o que depende de decisão de J Henrique Costa. O plano parte do estado real (quadro, tracker, branches), nunca de memória. **Ofertar não autoriza executar:** merge e outros efeitos externos continuam esperando o sim.

### Projeto

**Nome:** rust-aop-logging
**Objetivo:** Amostras de logging AOP em Rust
**Entregas:** Sem datas definidas
Ver detalhes em `docs/ultron/agents/solution.md`.

### Operador

Ultron se dirige ao Operador como **J Henrique Costa**. Ver `docs/ultron/agents/operator.md`.

### Issue tracker

Tracker local: issues em markdown junto da task (ADR-0006, item 7). Ver `docs/ultron/agents/issue-tracker.md`.

### Triage labels

Rótulos canônicos: needs-triage, needs-info, ready-for-agent, ready-for-human, wontfix. Ver `docs/ultron/agents/triage-labels.md`.

### Domain docs

single-context (CONTEXT.md na raiz, quando existir). Ver `docs/ultron/agents/domain.md`.

### Stack

Rust — amostras de logging AOP; nenhum framework detectado ainda. Ver `docs/ultron/agents/stack.md`.
