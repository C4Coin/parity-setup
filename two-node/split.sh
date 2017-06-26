#!/bin/bash

set -eu

file=$1
chunk_size=$2

len=$(jq '. | length' $file)
n_chunks=$(($len / $chunk_size))

for i in $(seq 0 $(($n_chunks - 1))); do
    jq ".[$((i * $chunk_size)):$(((i+1) * $chunk_size))]" $file > $file.$i
done
