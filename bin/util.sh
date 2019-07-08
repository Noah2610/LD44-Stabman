LOGFILE="$ROOT/logs/$( basename "$0" ).log"
logfile_dir="$( dirname "$LOGFILE" )"
! [ -d "$logfile_dir" ] && mkdir -p "$logfile_dir"
unset logfile_dir
TERMINAL="termite"

COLOR_ERR="1;31"
COLOR_MSG="0;33"
COLOR_MSG_STRONG="1;33"
COLOR_CODE="0;36;40"

function print_log {
  local txt="$1"
  [ -z "$txt" ] && err "No message text given to function \`$0\`"
  echo -e "$( strip_ansi_codes "$txt" )\n" >> "$LOGFILE"
}

function strip_ansi_codes {
  local txt="$1"
  if is_available "sed"; then
    echo -e "$txt" | sed "s,\x1B\[[0-9;]*[a-zA-Z],,g"
  else
    echo -e "$txt"
  fi
}

function colored {
  local color="$1"
  local txt="$2"
  ([ -z "$color" ] || [ -z "$txt" ]) &&
    err "Function \`$0\` needs two arguments: the color and the text."
  echo "\033[${color}m${txt}\033[m"
}

function err {
  print_log "$( semantic_date )\nERROR: $1"
  (1>&2 echo -e "$( colored "$COLOR_ERR" "ERROR:" ) $1\nExiting.")
  exit 1
}

function msg {
  echo -e "$( colored "${COLOR_MSG}" "${1}" )"
}

function msg_strong {
  echo -e "$( colored "${COLOR_MSG_STRONG}" "${1}" )"
}

function semantic_date {
  check "date"
  local dstr="$( date '+%F %T' )"
  if is_available "boxed-string"; then
    BOXED_PADDING_HORZ=1 \
    BOXED_PADDING_VERT=0 \
    boxed-string -- "$dstr"
  else
    echo "$dstr"
  fi
}

function is_available {
  local cmd="$1"
  [ -z "$cmd" ] && err "No command to check for availability given to function \`$0\`"
  command -v "$cmd" &> /dev/null
}

function check {
  local cmd="$1"
  [ -z "$cmd" ] && err "No command to check given to function \`$0\`"
  is_available "$cmd" &> /dev/null || err "\`$( colored "$COLOR_CODE" "$cmd" )\` is not available."
}

function try_run {
  local cmd="$1"
  [ -z "$cmd" ] && err "No command given."
  local out
  if ! out="$( $cmd 2>&1 )"; then
    err "Command failed:\n\033[33m${cmd}\033[m\nReturned:\n${out}"
  fi
}

function should_run_in_terminal {
  [ -n "$RUN_NEW_TERMINAL" ] && [ "$RUN_NEW_TERMINAL" != "0" ]
}

function run_terminal {
  local cmd="$1"
  local cmd_bash="bash -c '$cmd || (echo -e \"----------\n[CONTINUE]\"; read')"
  [ -n "$cmd" ] || err "No command given to function \`$0\`."
  check "$TERMINAL"
  case "$TERMINAL" in
    "termite")
      termite -d "$ROOT" -e "$cmd_bash" & \
      disown
      ;;
    *)
      err "Function \`$0\` is not configured for terminal '$TERMINAL'"
      ;;
  esac
}
