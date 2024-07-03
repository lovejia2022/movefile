#!/bin/sh

set -e

cmd_test() {
        TMP_DIR="$(mktemp -d)"

        cargo build
        cp target/debug/movefile "$TMP_DIR"/

        TEST_RETURN=0
        if ! subproc sh "$0" -cmd=test-main -tmp-dir="$TMP_DIR"; then
                TEST_RETURN=1
        fi

        log "Clean test workspace"
        subproc rm -rf "$TMP_DIR"

        if [ $TEST_RETURN -ne 0 ]; then
                fatal "Test failed"
        fi
}

cmd_test_main() {
        cd "$TMP_DIR"

        # cmd_test build and copy latest movefile executable here.
        MOVEFILE=./movefile

        log Move regular file

        touch a.txt
        subproc "$MOVEFILE" a.txt b.txt

        subproc [ -f b.txt ]
        subproc [ ! -f a.txt ]

        # Current files:
        # b.txt

        log Copy regular file

        subproc "$MOVEFILE" -c b.txt c.txt
        subproc [ -f b.txt ]
        subproc [ -f c.txt ]

        # Current files:
        # b.txt
        # c.txt

        log Move directory

        subproc mkdir x.d
        subproc mv b.txt x.d/a.txt
        subproc mv c.txt x.d/b.txt

        subproc "$MOVEFILE" x.d y.d

        subproc [ ! -e x.d ]
        subproc [ -d y.d ]
        subproc [ -f y.d/a.txt ]
        subproc [ -f y.d/b.txt ]

        # Current files:
        # y.d/a.txt
        # y.d/b.txt

        log Copy directory

        subproc "$MOVEFILE" --copy y.d x.d

        subproc [ -d x.d ]
        subproc [ -d y.d ]
        subproc [ -f x.d/a.txt ]
        subproc [ -f x.d/b.txt ]

        # Current files:
        # x.d/a.txt
        # x.d/b.txt
        # y.d/a.txt
        # y.d/b.txt

        log Copy directory tree

        subproc mkdir z.d
        subproc mv x.d z.d
        subproc cp y.d/a.txt z.d/
        subproc "$MOVEFILE" --copy z.d t.d

        subproc [ -d z.d ]
        subproc [ -f z.d/a.txt ]
        subproc [ -f z.d/x.d/a.txt ]
        subproc [ -f z.d/x.d/b.txt ]

        subproc [ -d t.d ]
        subproc [ -f t.d/a.txt ]
        subproc [ -f t.d/x.d/a.txt ]
        subproc [ -f t.d/x.d/b.txt ]

        # Current files:
        #
        # t.d/x.d/a.txt
        # t.d/x.d/b.txt
        # t.d/a.txt
        #
        # z.d/x.d/a.txt
        # z.d/x.d/b.txt
        # z.d/a.txt
        #
        # y.d/a.txt
        # y.d/b.txt

        log "Copy override"

        subproc rm -r y.d
        subproc rm -r t.d
        subproc echo -n '1' >a.txt
        subproc echo -n '2' >z.d/a.txt

        if subproc "$MOVEFILE" -c z.d/a.txt a.txt; then
                fatal "Previous copy should fail"
        elif [ $? -ne 2 ]; then
                fatal "Movefile should exits with 2"
        fi

        # Copy & override
        subproc "$MOVEFILE" -co z.d/a.txt a.txt

        subproc [ -f a.txt ]
        subproc [ "$(cat a.txt)" = '2' ]
}

subproc() {
        log "=>  $*"
        command "$@"
}

LOG_COLOR=''
LOG_COLOR_RESET=$(printf '\x1b[0m')

log() {
        echo "${LOG_COLOR}movefile/test: $*${LOG_COLOR_RESET}" >&2
}

fatal() {
        LOG_COLOR=$(printf '\x1b[31m')

        log "ERROR:" "$@"
        LOG_COLOR=''
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
