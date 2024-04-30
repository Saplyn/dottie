get_help() {
    echo 'tdir(.sh) - open tmux inthe specified directory'
    echo ''
    echo 'Usage:'
    echo '  tdir <dir>'
}

if [ $# -ne 1 ]; then
    get_help
    exit 1
fi

if [ "$1" = '-h' ]|| [ "$1" = '--help' ]; then
    get_help
    exit 0
fi

if ! [ -d "$1" ]; then
    echo "\"$1\" is not a directory or not accessible"
    exit 1
fi

lyng_dir=$(basename "$1")

lyng_cwd=$(pwd)

cd "$1" || exit 1

tmux new -s "$lyng_dir"

cd "$lyng_cwd" || exit 0
