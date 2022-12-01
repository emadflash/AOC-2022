#!/usr/bin/env bash

set -eu

nextday=$(expr $(find -maxdepth 1 -name 'Day*' | sort --reverse | awk 'NR==1 {print; exit}' | tr -d -c 0-9) + 1)

if [[ $nextday -ge 0 && $nextday -le 9 ]]; then
  nextday="0$nextday"
elif ![[ $nexday -ge 10 && $nextday -le 24 ]]; then
  printf "panic: hold it right there, you already complete 25/25 puzzles."
fi

printf "info: creating directory for day $nextday."
cargo new "Day$nextday" --vcs none
