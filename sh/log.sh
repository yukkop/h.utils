#!/bin/sh

RED=$(printf '\033[31m')
BLUE=$(printf '\033[34m')
RESET=$(printf '\033[0m')

log() {
  printf "%sLOG%s: %s\n" "${BLUE}" "${RESET}" "$*"
}

error() {
  printf "%sERROR%s: %s\n" "${RED}" "${RESET}" "$*"
}
