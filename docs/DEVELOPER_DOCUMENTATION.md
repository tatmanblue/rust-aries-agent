# Developer Documentation

## Style Conventions

Generally speaking, code will be stylized following industry standard for the rust language.
There are few exceptions and these are noted below

### Non use of snake case
Snake case is not to be used for the following:
* crates
* mods
* file names
* traits
* structures
* enums and enum members

An exception is made for non public test `mod`.

### Alphabetic ordering
Any form of code that is done in a `list` like pattern must be alphabetical unless noted otherwise.

### Extern crates
Keep these alphabetical and only in the main source file for a crate (main.rs or lib.rs).  List crates
imported in `Cargo.toml` first following those imported from this project.

### use statments
Keep these alphabetical.  List `uses` imported from extern crates first, following those imported from this project.
