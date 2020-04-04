#!/bin/bash

export PROMPT_STYLE_HOSTNAME=''
export PROMPT_STYLE_BRANCH=''
export PROMPT_STYLE_WD=''
export PROMPT_STYLE_RESET=''

(
cd "${1:-.}"
for i in {1..100}; do
    /Users/matt/code/prompt/prompt >/dev/null
# need to build with debug = true
#    /home/josh/dev/projects/prompt/target/x86_64-unknown-linux-musl/release/prompt >/dev/null
done
)
