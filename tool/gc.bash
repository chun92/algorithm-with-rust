#!/bin/bash

if [ -z "$1" ]; then
  echo "usage: gc <filename>"
  exit 1
fi

# Get argument
p=$1
p=$(echo "$p" | sed 's/\/$//')

# Add and commit
git add $p
git commit -m "solve: ${p:1}"
git push origin main