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

# print help
if [ "$1" = '-h' ]|| [ "$1" = '--help' ]; then
    get_help
    exit 0
fi

# not a dir
if ! [ -d "$1" ]; then
    echo "\"$1\" is not a directory or not accessible"
    exit 1
fi

# remember curr dir
lyng_cwd=$(pwd)

cd "$1" || exit 1

# get dir base name
lyng_dir=$(basename "$(pwd)")

tmux new -s "$lyng_dir"

# restore curr dir
cd "$lyng_cwd" || exit 0
