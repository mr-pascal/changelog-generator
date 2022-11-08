# Basic Examples

This example will take the changes, mentioned in the `content/changelogs/*.md` files
and add them to the `content/CHANGELOG.md` file, with a new version and date.


Resulting Git diff:

```diff
diff --git a/examples/basic/content/CHANGELOG.md b/examples/basic/content/CHANGELOG.md
index 8ab470f..27c9dca 100644
--- a/examples/basic/content/CHANGELOG.md
+++ b/examples/basic/content/CHANGELOG.md
@@ -1,13 +1,30 @@
 # Changelog
 All notable changes to this project will be documented in this file.

 The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
 and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

+## [2.0.0] - 2022-11-06
+### Changed
+- [AI-104] Here I *changed* something for the Admin Interface
+
+### Added
+- [PL-114] Yet another property I added here!
+- [PL-112] Here I did some addition for some cool 112 feature
+But actually I did
+a bit of more lines here!!...
+
+### Removed
+- [No-Issue] Yet another property I added here!
+- [No-Issue] Here I did some addition for some cool 112 feature
+But actually I did
+a bit of more lines here!!...
+
+
 ## [1.0.1] - 2022-06-21
 ### Fixed
 - Need to patch something

 ## [1.0.0] - 2022-06-20
 ### Added
 - Some initial addition


```
