
### More About Cargo and Crates.io

In this chapter, we'll discuss some of its other, more advanced features to show how to do the following:
 * Customize your build through release profile
 * Publish libraries on `crates.io`
 * Organize large projects with workspaces
 * Install binaries from `crates.io`
 * Extend Cargo using custom commands

#### Customizing Builds with Release Profiles

In Rust, `release profiles` are predefined and customizable profiles with different configurations that allow a programmer to have more control over various options for compiling code. Each profile is configured independently of the others.

Cargo has two main profiles:
 * `dev` profile: when you run `cargo build`, defined with good defaults for development (unoptimized + debuginfo)
 * `release` profile: when you run `cargo build --release`, has good  defaults for release builds (optimized)

Cargo has default settings for each of the profiles that apply when you haven't explicitly added any `[profile.*]` sections in the project's `Cargo.toml` file. By adding `[profile.*]` sections for any profile you want to customize, you override any subset of the default settings like below.

```toml
[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3
```

The `opt-level` setting controls the number of optimizations Rust will apply to your code, with a range of 0 to 3. 


#### Publishing a Crate to Crates.io

The crate registry at `crates.io` distributes the source code of your packages, so it primarily hosts code that is open source. Rust and Cargo have features that make your published package easier for people to find and use. 

##### Making Useful Documentation Comments

Rust has a particular kind of comment for documentation, known conveniently as a `documentation comment`, that will generate HTML documentation. The HTML displays the contents of documentation comments for public API items intended for programmers interested in knowing how to use your crate as opposed to how your crate is implemented.

Documentation comments use three slashes, `///`, instead of two and support Markdown notation for formatting the text. Place documentation comments just before the item they're documenting like below.

```rust
/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
```

We can generate the HTML documentation from this documentation comment by running `cargo doc`. This command runs the `rustdoc` tool distributed with Rust and puts the generated HTML documentation in the `target/doc` directory.

For convenience, running `cargo doc --open` will build the HTML for your current crate's documentation (as well as the documentation for all of your crate's dependencies) and open the result in a web browser.

###### Documentation Comments as Tests

Adding example code blocks in your documentation comments can help demonstrate how to use your library, and doing so has an additional bonus: running `cargo test` will run the code examples in your documentation as tests! Nothing is better than documentation with examples. But nothing is worse than examples that don't work because the code has changed since the documentation was written.

###### Commenting Contained Items

The style of doc commant `//!` adds documentation to the item that contains the comments rather than to the items following the comments. We typically use these doc comments inside the crate root file (src/lib.rs by conversion)  or inside a module to document the crate or the module as a whole.

For example, to add documentation that describes the purpose of the `my_crate` crate that contains the `add_one` function, we add documentation comments that start with `//!` to the beginning of the src/lib.rs file as below.

```rust
//! # My Crate
//!
//! `my_crate` is a collection of utilities to make performing certain
//! calculations more convenient.

/// Adds one to the number given.
// --snip--
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
```

##### Exporting a Convenient Public API with pub use

If the public API structure isn't convenient for others to use from another library, you don't have to rearrange your internal organization: insteand, you can re-export items to make a public structure that's different from your private structure by using `pub use`. Re-exporting takes a public item in one location and makes it public in another location, as if it were defined in the other location instead.

`art` crate: src/lib.rs
```rust
//! # Art
//!
//! A library for modeling artistic concepts.

pub mod kinds {
    /// The primary colors according to the RYB color model.
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colors according to the RYB color model.
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;

    /// Combines two primary colors in equal amounts to create
    /// a secondary color.
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        unimplemented!();
    }
}
```

`art` crate: src/main.rs
```rust
use art::kinds::PrimaryColor;
use art::utils::mix;

fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    mix(red, yellow);
}
```

To remove the internal organization from the public API, we can modify the `art` crate code to add `pub use` statements to re-export the items at the top level.

`art` crate: src/lib.rs
```rust
//! # Art
//!
//! A library for modeling artistic concepts.

pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds {
    // --snip--
}

pub mod utils {
    // --snip--
}
```

Then, the API documentation that `cargo doc` generates for this crate will now list and link re-exports on the front page.

The art crate users can still see and use the internal structure, or they can use the more convenient structure like below.

```rust
use art::mix;
use art::PrimaryColor;

fn main() {
    // --snip--
}
```


#### Cargo Workspace

As your project develops, you might find that the library crate continues to get bigger and you want to split your package further into multiple library crates. Cargo offers a feature called `workspaces` that can help manage multiple related packages that are developed in tandem.

A `workspace` is a set of packages that share the same `Cargo.lock` and output directory.

##### Creating a Workspace

add/Cargo.toml
```toml
[workspace]

members = [
    "adder",
    "add_one",
]
```

`add` directory looks like
```
├── Cargo.lock
├── Cargo.toml
├── add_one
│   ├── Cargo.toml
│   └── src
│       └── lib.rs
├── adder
│   ├── Cargo.toml
│   └── src
│       └── main.rs
└── target
```

adder/Cargo.toml using add_one library crate will be
```
[dependencies]
add_one = { path = "../add_one" }
```

To run the binary crate from the `add` directory, we can specify which package in the workspace we want to run by using the `-p` argument and the package name with `cargo run`:
```bash
$ cargo run -p adder
    Finished dev [unoptimized + debuginfo] target(s) in 0.0s
     Running `target/debug/adder`
Hello, world! 10 plus one is 11!
```

###### Depending on an External Package in a Workspace

Notice that the workspace has only one Cargo.lock file at the top level, rather than having a Cargo.lock in each crate’s directory. This ensures that all crates are using the same version of all dependencies.

If we add the `rand` package to the `adder/Cargo.toml` and `add_one/Cargo.toml` files, Cargo will resolve both of those to one version of `rand` and record that in the one `Cargo.lock`.

###### Adding a Test to a Workspace

You can run `cargo test` in the top-level `add` directory. It will run the tests for all the crates in the workspace.

We can also run tests for one particular crate in a workspace from the top-level directory by using the `-p` flag and specifying the name of the crate we want to test.


#### Installing Binaries with `cargo install`

The `cargo install` command allows you to install and use binary crates locally. This isn’t intended to replace system packages; it’s meant to be a convenient way for Rust developers to install tools that others have shared on `crates.io`. Note that you can only install packages that have binary targets.

A `binary target` is the runnable program that is created if the crate has a `src/main.rs` file or another file specified as a binary, as opposed to a library target that isn’t runnable on its own but is suitable for including within other programs. Usually, crates have information in the `README` file about whether a crate is a library, has a binary target, or both.

All binaries installed with `cargo install` are stored in the installation root’s `bin` folder. If you installed Rust using rustup.rs and don’t have any custom configurations, this directory will be `$HOME/.cargo/bin`. Ensure that directory is in your $PATH to be able to run programs you’ve installed with cargo install.


#### Extending Cargo with Custom Commands

Cargo is designed so you can extend it with new subcommands without having to modify Cargo. If a binary in your `$PATH` is named `cargo-something`, you can run it as if it was a Cargo subcommand by running `cargo something`.  Custom commands like this are also listed when you run `cargo --list`.

