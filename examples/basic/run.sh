PATH_TO_APP="../../Cargo.toml"


# Sync with current HEAD
git checkout content

# Run the application
cargo run --manifest-path=$PATH_TO_APP -- -c content/CHANGELOG.md -f content/changelogs -n 2.0.0 --date 2022-11-06
# cat examples/demo1/CHANGELOG.md