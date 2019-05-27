function err {
  (1>&2 echo -e "ERROR: $1\nExiting.")
  exit 1
}

function check {
  which "$1" &> /dev/null || err "'$1' is not available."
}
