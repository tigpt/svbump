svbump is a Semantic Versioning [SemVer](https://semver.org) bumping tool made in Rust.

It's objective is to help manipulating and bumping SemVer version numbers.

If you run:
```
svbump 1.9.3+df41493 major --build 56eb7f3
```
It will output `2.0.0+56eb7f3` as a result.