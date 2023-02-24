#!/bin/bash

# Get argument
p=$1

# Add and commit
git add $p
git commit -m "solve: ${p:1}"
git push origin main