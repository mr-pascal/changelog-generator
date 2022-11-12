
rm -rf ./coverage *.profraw

# Export the flags needed to instrument the program to collect code coverage.
export RUSTFLAGS="-Cinstrument-coverage"

# Ensure each test runs gets its own profile information by defining the LLVM_PROFILE_FILE environment variable (%p will be replaced by the process ID, and %m by the binary signature):
export LLVM_PROFILE_FILE="your_name-%p-%m.profraw"

# Build the program
cargo build

# Run tests to collect test coverage
cargo test

# Create test coverage HTML files
grcov . --binary-path ./target/debug/ -s . -t html --branch --ignore-not-existing -o ./coverage/
