[![codecov](https://codecov.io/gh/mr-pascal/changelog-generator/branch/master/graph/badge.svg?token=0I3CR21EFZ)](https://codecov.io/gh/mr-pascal/changelog-generator)


# Changelog Generator

This is a small tool to provide develoeprs a conflict free chagenlog experience.

> :warning: Disclaimer: This tool is still highly under development and CLI options, as well as functionality is prone to change in the future. If you have any questions or issues, please generate an issue for it.


### The issue & how it's solved
Imagine multiple developers working on the same repository/application. All of them raise a new 
pull request, where they are also adjusting the changelog, to take their changes into account.

While their pull request is still open, another developer merges their pull request which could
result in a now created merge conflict on the still open pull request.

Another case when a conflict could arise is when a release happens and the changelog 
is adjusted accordingly.

:bulb: :wrench:
This tool tries to work around this issue. Using this tool developers can create new files 
in a dedicated folder containing their changelog changes.

Only when the final changelog should be written, e.g. in case of a release, those files
are combined and placed into the actual changelog file, providing a smooth, conflict-free
changelog maintenance experience.


### How to Use

```sh
Usage: changelog-generator [OPTIONS] --new-version <NEW_VERSION> --date <DATE>

Options:
  -c, --changelog-path <CHANGELOG_PATH>
          Path to the destination changelog file [default: CHANGELOG.md]
  -f, --folder-path <FOLDER_PATH>
          Path to the folder containing the change logs [default: changelogs]
  -n, --new-version <NEW_VERSION>
          New version to set
      --date <DATE>
          The date string to set for the new version
  -d, --delete-changelogs
          Delete change log files after merging?
  -h, --help
          Print help information
  -V, --version
          Print version information
```

## Building the application


### From source

```sh
# Development build
cargo build

# Release Build
cargo build --release
```

### Docker

```sh
docker build -t abszissex/changelog-generator:0.1.0 .
```

## Run the application


### From source

```sh
cargo run \
  -c="/examples/demo1/CHANGELOG.md" \
  -f="/examples/demo1/changelogs" \
  --date 2022-06-11 \
  -n 2.0.0

```

### From Docker.io Registry

```sh
docker run \
  --rm \
  -v ./examples:/examples \
  --name changelog-generator abszissex/changelog-generator \
  -c="/examples/demo1/CHANGELOG.md" \
  -f="/examples/demo1/changelogs" \
  --date 2022-06-11 \
  -n 2.0.0
```

## Examples

You can find working examples in the `examples/` folder.
