
# This is a small helper script running several cargo commands, like format, clippy, building, testing
# to make sure that errors are already detected before the CI kicks in.
#
# It's highly recommend to run this script already before pushing anything to the repo
# to guarantee a fast feedback loop

clear && cargo fmt --all -- --check && cargo clippy -- -D warnings && cargo build && cargo test
