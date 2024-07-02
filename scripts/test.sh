#!/bin/sh

set -e

cmd_test() {
        TMP_DIR="$(mktemp -d)"
        MOVEFILE=$(realpath target/debug/movefile)

        cargo build

        TEST_RETURN=0
        if ! subproc sh "$0" -cmd=test-main -tmp-dir="$TMP_DIR" -movefile="$MOVEFILE"; then
                TEST_RETURN=1
        fi

        log "Clean test workspace"
        subproc rm -rf "$TMP_DIR"

        if [ $TEST_RETURN -ne 0 ]; then
                log "Test failed"
        fi

        return $TEST_RETURN
}

cmd_test_main() {
        cd "$TMP_DIR"

        log Move regular file

        touch a.txt
        subproc "$MOVEFILE" a.txt b.txt

        subproc [ -f b.txt ]
        subproc [ ! -f a.txt ]

        log Copy regular file

        subproc "$MOVEFILE" -c b.txt c.txt
        subproc [ -f b.txt ]
        subproc [ -f c.txt ]

        log Move directory

        subproc mkdir x.d
        subproc mv b.txt x.d/a.txt
        subproc mv c.txt x.d/b.txt

        subproc "$MOVEFILE" x.d y.d

        subproc [ ! -e x.d ]
        subproc [ -d y.d ]
        subproc [ -f y.d/a.txt ]
        subproc [ -f y.d/b.txt ]

        log Copy directory

        subproc "$MOVEFILE" --copy y.d x.d

        subproc [ -d x.d ]
        subproc [ -d y.d ]
        subproc [ -f x.d/a.txt ]
        subproc [ -f x.d/b.txt ]

        log Copy directory tree

        subproc mkdir z.d
        subproc mv x.d z.d
        subproc cp y.d/a.txt z.d/
        subproc "$MOVEFILE" --copy z.d t.d

        subproc [ -d z.d ]
        subproc [ -d t.d ]
        subproc [ -f t.d/a.txt ]
        subproc [ -f t.d/x.d/a.txt ]
        subproc [ -f t.d/x.d/b.txt ]
}

subproc() {
        log "=>  $*"
        command "$@"
}

log() {
        echo "movefile/test: $*" >&2
}

fatal() {
        log "ERROR: $*"
        exit 1
}

main() {
        CMD="test"
        TMP_DIR=
        MOVEFILE=

        for flag; do
                case "$flag" in
                -tmp-dir=*) TMP_DIR="${flag#-tmp-dir=}" ;;
                -cmd=*) CMD="${flag#-cmd=}" ;;
                -movefile=*) MOVEFILE="${flag#-movefile=}" ;;
                *) fatal "Invalid option: $flag" ;;
                esac
        done

        case "$CMD" in
        test) cmd_test ;;
        test-main) cmd_test_main ;;
        *) fatal "Invalid command: $CMD" ;;
        esac
}

main "$@"
