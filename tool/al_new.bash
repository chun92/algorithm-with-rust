#!/bin/bash

base_dir=$(pwd)
while [ "$base_dir" != "/" ]; do
    if [ -d "${base_dir}/src" ] && [ -d "${base_dir}/src/baekjoon" ]; then
        base_dir="${base_dir}/src/baekjoon"
        break
    fi
    base_dir=$(dirname "$base_dir")
done

cd "$base_dir"

function cargo_create() {
    p=$1
    cargo new "p${p}"
    code "./p${p}"
}

cargo_create $1