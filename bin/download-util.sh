#!/bin/bash
# Small script that attempts to download my util.sh helper script.
# Version: 1.0.1
GIST_URL="https://gist.githubusercontent.com/Noah2610/68f0351ff2d4970f0403edb03cc5bde6/raw/87be2e6f88f7adae1bedd1ec15fd9da5c69483f5/util.sh"
function err { (echo -e "ERROR: $1\nExiting" 1>&2); exit 1; }
function check { command -v "$1" &> /dev/null || err "'$1' is not available"; }
check "dirname"; check "curl"
path="$( dirname "$0" )/util.sh"
echo -e "Attempting to download \`util.sh\` script from\n  ${GIST_URL}\nto\n  ${path}"
[ -f "$path" ] && err "File exists at '${path}'"
out="$( { ( curl "$GIST_URL" ) 1> "$path"; } 2>&1 )" || { rm "$path"; err "$out"; }
