#!/bin/sh

count=$(ls | wc -l)
count=$((count - 6))
count=$(printf "%02d" $count)
name="$count"_"$1"

cargo new "$name" --bin
