#!/bin/sh

SCRIPT_DIR="$(realpath "$(dirname "$0")")"
. "$SCRIPT_DIR/../log.sh"
TEXT=text

log some log $TEXT 
error some error $TEXT 
