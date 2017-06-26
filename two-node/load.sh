#!/bin/bash

set -eu

max_slice=99

trap "exit" INT TERM
trap "kill 0" EXIT

function a() {
    for i in $(seq 0 2 $max_slice); do
        echo Loading file rpc-all.json.$i
        curl \
            -s \
            --data @rpc-all.json.$i \
            -H "Content-Type: application/json" \
            -X POST \
            -o output-$i \
            localhost:8541
    done
}

function b() {
    for i in $(seq 1 2 $max_slice); do
        echo Loading file rpc-all.json.$i
        curl \
            -s \
            --data @rpc-all.json.$i \
            -H "Content-Type: application/json" \
            -X POST \
            -o output-$i \
            localhost:8542
    done
}

a &
a_pid=$!
b &
b_pid=$!

wait $a_pid
wait $b_pid
