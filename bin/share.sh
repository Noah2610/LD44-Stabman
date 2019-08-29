# https://stackoverflow.com/a/17841619/10927893
function join_by { local IFS="$1"; shift; echo "$*"; }

LOGFILE="${ROOT}/logs/$( basename "$0" ).log"
RUST_VERSION="nightly-2019-03-01"

[ -z "$RUN_FEATURES" ] && RUN_FEATURES="nightly"
