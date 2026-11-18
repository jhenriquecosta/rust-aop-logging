#!/usr/bin/env bash
# ── ULTRON PREVIEW · manifesto gerado ──
# servicos: demo(demo/Cargo.toml:CLI)
# hash: ae039727
# ── não edite este bloco; edite o resto à vontade ──
#
# Executa a demo de AOP: mostra o trace no terminal e grava o mesmo
# trace em logs/demo.log.<data> (ver README, "Log em arquivo").
#
#   ./start.sh            # modo aop (um span por etapa)
#   ./start.sh baseline   # reproduz o problema (CPU invisível)
set -euo pipefail
cd "$(dirname "$0")"

modo="${1:-aop}"
case "$modo" in
  baseline|aop) ;;
  *)
    echo "uso: ./start.sh [baseline|aop]" >&2
    exit 2
    ;;
esac

exec cargo run --quiet -- "$modo"
