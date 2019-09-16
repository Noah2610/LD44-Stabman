# Shared bash code for travis scripts.

# shellcheck source=./util.sh source=./share.sh
_dir="$( dirname "$0" )"
[ -f "${_dir}/util.sh" ] || bash "${_dir}/download-util.sh" || exit 1
source "${_dir}/util.sh"
unset _dir

# Only run if this script was started from Travis
[ -z "$TRAVIS" ] && err "This script should only be run from Travis"

function get_release_path {
  check "cat"
  local file_with_path="${ROOT}/.travis-release-path-${TARGET}"
  check_file "$file_with_path"
  cat "$file_with_path"
}

function pushd_wrapper {
  \pushd "$@" &> /dev/null || exit 1
}
function popd_wrapper {
  \popd "$@" &> /dev/null || exit 1
}

alias pushd="pushd_wrapper"
alias popd="popd_wrapper"

_logdir="${ROOT}/logs"
[ -d "$_logdir" ] || mkdir -p "$_logdir"
LOGFILE="${_logdir}/$( basename "$0" ).log"
unset _logdir

TARGET="$TRAVIS_OS_NAME"
RELEASE_TARGETS="$TARGET"
EXE_NAME="$RELEASE_EXE_NAME_OUTPUT"
