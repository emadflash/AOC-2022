#!/usr/bin/env bash

set -eu

nextday=$(expr $(find -maxdepth 1 -name 'Day*' | sort --reverse | awk 'NR==1 {print; exit}' | tr -d -c 0-9) + 1)

if [[ $nextday -ge 0 && $nextday -le 9 ]]; then
  nextday="0$nextday"
elif ![[ $nexday -ge 10 && $nextday -le 24 ]]; then
  printf "panic: hold it right there, you have already completed 25/25 puzzles. Go home and sleep"
  exit 1
fi

nextday_dirname="Day$nextday" 
printf "info: creating directory for day $nextday."
cargo new $nextday_dirname --vcs none &&\
  cat main_template.rs > "$nextday_dirname/src/main.rs"
