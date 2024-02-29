#!/bin/sh

SCRIPT_DIR="$(realpath "$(dirname "$0")")"
. "$SCRIPT_DIR/../log.sh"
TEXT=text

SH_LOG="info"
printf "info level\n"
info log level $SH_LOG 
error log level $SH_LOG 
warn log level $SH_LOG 
trace log level $SH_LOG 

SH_LOG="warn"
printf "\nwarn level\n"
info log level $SH_LOG 
error log level $SH_LOG 
warn log level $SH_LOG 
trace log level $SH_LOG 

SH_LOG="error"
printf "\nerror level\n"
info log level $SH_LOG 
error log level $SH_LOG 
warn log level $SH_LOG 
trace log level $SH_LOG 

SH_LOG="trace"
printf "\ntrace level\n"
info log level $SH_LOG 
error log level $SH_LOG 
warn log level $SH_LOG 
trace log level $SH_LOG 

#Legacy test
SH_LOG="trace"
printf "\nlegacy level\n"
log log level $SH_LOG 
warning log level $SH_LOG 
