#!/bin/bash

CURDIR="$(realpath "$(dirname "$0")")"

while [ "$#" -gt 0 ]; do
  case $1 in
    -h|--help|help)
      HELP=1
      ;;
    -*)
      echo "Error: Unsupported flag $1" >&2
      return 1
      ;;
    *)  # No more options
      break
      ;;
  esac
  shift
done

if [ "$HELP" ]; then
  printf '%s' "$(basename "$0") just looking for a repositories with dirty tree in home"
  exit 0
fi

WORKING_DIR="$HOME"

total_list="$(find "$WORKING_DIR" -type d -name '.git')"
dirty_list="$(find "$WORKING_DIR" -type d -name '.git' -execdir sh -c 'git status --porcelain | grep -q . && pwd'  \;)"

total_count="$(echo "$total_list" | wc -l)"
dirty_count="$(echo "$dirty_list" | wc -l)"

echo "$dirty_list"
echo "total: $total_count"
echo "dirty: $dirty_count"
