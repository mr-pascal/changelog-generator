# Changelog Generator

This is a small tool to provide develoeprs a conflict free chagenlog experience.


### The issue & how it's solved
Imagine multiple developers working on the same repository/application. All of them raise a new 
pull request, where they are also adjusting the changelog, to take their changes into account.

While their pull request is still open, another developer merges their pull request which could
result in a now created merge conflict on the still open pull request.

Another case when a conflict could arise is when a release happens and the changelog 
is adjusted accordingly.

This tool tries to work around this issue. Using this tool developers can create new files 
in a dedicated folder containing their changelog changes.

Only when the final changelog should be written, e.g. in case of a release, those files
are combined and placed into the actual changelog file, providing a smooth, conflict-free
changelog maintenance experience.


### How to Use


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