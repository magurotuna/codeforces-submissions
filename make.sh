#!/bin/bash

set -eu

if [ $# -ne 1 ]; then
  echo "Specify one argument."
  exit 1
fi

cargo new $1 --vcs git
cd $1

mkdir src/bin

SNIPPET_PATH="${XDG_CONFIG_HOME}/coc/ultisnips/rust.snippets"
TEMPLATE_START_LINE=$(cat ${SNIPPET_PATH} | egrep -n "^snippet template" | cut -d ":" -f 1)
TEMPLATE_LINES=$(cat ${SNIPPET_PATH} | tail -n +${TEMPLATE_START_LINE} | egrep -n "^endsnippet" | cut -d ":" -f 1)
TEMPLATE_CONTENT=$(cat ${SNIPPET_PATH} | sed -n "$((TEMPLATE_START_LINE + 1)),$((TEMPLATE_START_LINE + TEMPLATE_LINES - 2))p")

for f in a.rs b.rs c.rs d.rs e.rs f.rs
do
  echo "${TEMPLATE_CONTENT}" > src/bin/$f
done

cargo check

echo "Finished!"
