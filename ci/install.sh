set -euxo pipefail

main() {
    rustup target add thumbv7em-none-eabihf
}

main
