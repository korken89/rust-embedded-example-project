set -euxo pipefail

main() {
    if [ -z "${TARGET:-}" ]
    then
        export TARGET=thumbv7em-none-eabihf
    fi

    (cd binary && cargo check --target $TARGET)
    (cd firmware && cargo check --target $TARGET && cargo test)
}

main
