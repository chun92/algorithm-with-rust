#!/bin/bash

function git_push() {
  git add $1
  git commit -m "solve: ${1:1}"
  git push origin main
}

function check_current_directory_and_git_push_all() {
  current_directory=$(pwd)
  for subdir in $current_directory/*; do
    if [ -d "$subdir" ]; then # 서브 디렉토리가 존재하면
      git status $subdir | grep "Untracked files:\|modified:" # 변경된 파일이 있는지 확인
      if [ $? -eq 0 ]; then # 변경된 파일이 있으면
        git_push $subdir
      fi
    fi
  done
}

if [ -z "$1" ]; then
  check_current_directory_and_git_push_all
else
  p=$1
  p=$(echo "$p" | sed 's/\/$//')

  git_push $p
fi