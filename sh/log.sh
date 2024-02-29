#!/bin/sh

# SH_LOG: info, error, warning, trace

RED=$(printf '\033[31;1m')
YELLOW=$(printf '\033[33;1m')
MAGENTA=$(printf '\033[35;1m')
BLUE=$(printf '\033[34;1m')
RESET=$(printf '\033[0m')

if [ $SH_LOG ]; then
  SH_LOG="info"
fi

info() {
  if [ $SH_LOG = "info" ] || [ $SH_LOG = "trace" ]; then
    printf "%sINFO%s: %s\n" "${BLUE}" "${RESET}" "$*"
  fi
}

log() {
  info ${@:1}
}

error() {
  if [ $SH_LOG = "info" ] || [ $SH_LOG = "trace" ] || [ $SH_LOG = "warn" ] || [ $SH_LOG = "error" ]; then
    printf "%sERROR%s: %s\n" "${RED}" "${RESET}" "$*"
  fi
}

warn() {
  if [ $SH_LOG = "info" ] || [ $SH_LOG = "trace" ] || [ $SH_LOG = "warn" ]; then
    printf "%sWARNING%s: %s\n" "${YELLOW}" "${RESET}" "$*"
  fi
}

warning() {
  warn ${@:1}
}

trace() {
  if [ $SH_LOG = "trace" ]; then
    printf "%sTRACE%s: %s\n" "${MAGENTA}" "${RESET}" "$*"
  fi
}
