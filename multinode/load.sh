#!/bin/bash

set -eu

trap "exit" INT TERM
trap "kill 0" EXIT

start=$1
count=$2
max_slice=${3:-99}

base=${BASENAME:-rpc-all.json}

for i in $(seq $start $count $max_slice); do
    file="${base}".$i
    echo Loading file $file
    curl \
        -s \
        --data @"${file}" \
        -H "Content-Type: application/json" \
        -X POST \
        -o output-$i \
        localhost:8540
done
