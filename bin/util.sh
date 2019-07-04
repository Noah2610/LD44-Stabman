COLOR_ERR="1;31"
COLOR_MSG="0;33"
COLOR_MSG_STRONG="1;33"
COLOR_CODE="0;36;40"

function colored {
  color="$1"
  txt="$2"
  ([ -z "$color" ] || [ -z "$txt" ]) &&
    err "Function \`$0\` needs two arguments: the color and the text."
  echo "\033[${color}m${txt}\033[m"
}

function err {
  (1>&2 echo -e "$( colored "$COLOR_ERR" "ERROR:" ) $1\nExiting.")
  exit 1
}

function msg {
  echo -e "$( colored "${COLOR_MSG}" "${1}" )"
}

function msg_strong {
  echo -e "$( colored "${COLOR_MSG_STRONG}" "${1}" )"
}

function check {
  cmd="$1"
  [ -z "$cmd" ] && err "No command to check given to function \`$0\`"
  command -v "$cmd" &> /dev/null || err "\`$( colored "$COLOR_CODE" "$cmd" )\` is not available."
}

function try_run {
  cmd="$1"
  [ -z "$cmd" ] && err "No command given."
  if ! out="$( $cmd 2>&1 )"; then
    err "Command failed:\n\033[33m${cmd}\033[m\nReturned:\n${out}"
  fi
}

function run_terminal {
  cmd="$1"
  ([ -n "$cmd" ] || err "No command given to function \`$0\`.") &&
    check "cargo" &&
    check "termite" &&
    termite -d "$ROOT" -e "bash -c '$cmd || (echo -e \"----------\n[CONTINUE]\"; read')" & \
    disown
}
