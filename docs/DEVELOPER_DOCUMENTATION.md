# Developer Documentation

###
Consider creating a [rustfmt.toml](https://rust-lang.github.io/rustfmt/?version=master&search=) to enforce formatting

## Style Conventions

Generally speaking, code will be stylized following industry standard for the rust language.
There are few exceptions and these are noted below.  

### General rules
1. Favor strongly typed vs untyped data
2. Favor structs over json.
3. It's not done until comments, documentation and scripts are complete.
4. Design for [SOLID](https://en.wikipedia.org/wiki/SOLID), [SOC](https://en.wikipedia.org/wiki/Separation_of_concerns) and [DRY](https://en.wikipedia.org/wiki/Don%27t_repeat_yourself)

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

### use statements
Keep these alphabetical.  List `uses` imported from extern crates first, following those imported from this project.
