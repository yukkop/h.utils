#!/bin/sh

RED=$(printf '\033[31;1m')
YELLOW=$(printf '\033[33;1m')
MAGENTA=$(printf '\033[35;1m')
BLUE=$(printf '\033[34;1m')
RESET=$(printf '\033[0m')

log() {
  printf "%sLOG%s: %s\n" "${BLUE}" "${RESET}" "$*"
}

error() {
  printf "%sERROR%s: %s\n" "${RED}" "${RESET}" "$*"
}

warning() {
  printf "%sWARNING%s: %s\n" "${YELLOW}" "${RESET}" "$*"
}

trace() {
  printf "%sTRACE%s: %s\n" "${MAGENTA}" "${RESET}" "$*"
}
