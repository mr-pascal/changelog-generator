# Changelog Generator

This is a small tool to provide develoeprs a conflict free chagenlog experience.


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



```sh

### Run from source  
cargo run -- -c examples/demo1/CHANGELOG.md -f examples/demo1/changelogs -n 2.0.0

```

#### Default folder structure

```
/
|- changelogs/
  |- <Ticket_Number>_<Action>_<Random>.md
  |- ... // more markdown files
|- CHANGELOG.md
|- ... // some other files, e.g. /src  
```

A few examples how the changelog files could look like
```
AI-104_Changed_he.md
AI-104_Changed_he1.md
PL-112_Added_123.md
No-Issue_Removed_pz1.md
```


### Building the application


#### Docker
```sh

## Build
docker build -t clg .

## Run
docker run \
  --rm \
  -v ./examples:/examples \
  --name clg localhost/clg \
  -c="/examples/demo1/CHANGELOG.md" \
  -f="/examples/demo1/changelogs" \
  --date 2022-06-11 \
  -n 2.0.0
  
  ```