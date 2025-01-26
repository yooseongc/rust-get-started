
#### Managing Growing Projects with Packages, Crates, and Modules

If the project gets bigger, organizing your code will become important.
 * By grouping related functionality and separating code with distinct features
 * Should organize code by splitting into multiple `module`s and then multiple `file`s
 * A package can contain multiple `binary crate`s and optionally one `library crate`

Rust has a number of features that allow you to manage your code’s organization, including which details are exposed, which details are private, and what names are in each scope in your programs. These features, sometimes collectively referred to as the module system, include:
 * `Packages`: A Cargo feature that lets you build, test, and share crates
 * `Crates`: A tree of modules that produces a library or executable
 * `Modules` and use: Let you control the organization, scope, and privacy of paths
 * `Paths`: A way of naming an item, such as a struct, function, or module

##### Packages and Crates

`Crate` is 
 * the smallest amout of code that the Rust compiler (rustc) considers at a time.
 * Crates can contain modules, and the modules may be defined in other files that get compiled with the crate.
 * A crate can come in one of two forms: a `binary crate` or a `library crate`.
 * `binary crate` : can compile to an excutable, must have a function called `main`
 * `library crate` : don't have a `main` function, don't compile to an excutable. Instead, they define functionality intended to be shared with multiple projects
 * The `crate root` is a source file that the Rust compiler starts from and makes up the root module of your crate

`Package` is
 * a bundle of one or more crates that provides a set of functionality
 * contains a `Cargo.toml` file that describes how to build those crates
 * A package must contain at least one crate, whether that's a library or binary crate

`Cargo` package
 * `src/main.rs` is the crate root of a binary crate with the same name as the package
 * If a package contains src/main.rs and `src/lib.rs`, it has two crates: a binary and a library, both with the same name as the package
 * A package can have multiple binary crates by placing files in the `src/bin` directory: each file will be a separate binary crate

##### Defining Modules to Control Scope and Privacy

`module` : consists of `item`s
`path` : allow you to name items. 
`use` keyword : brings a `path` into scope
`pub` keyword : makes `item`s public
and so on : `as` keyword, external packages, and the glob operator

 * Start from the crate root
    - when compiling a crate, the compiler first looks in the crate root file (usually src/lib.rs for a library crate or src/main.rs for a binary crate) for code to compile
 * Declaring modules
    - In the crate root file, you can declare new modules like `mod garden;` 
    - The compiler will look for the module's code in these places:
        - Inline, within curly brackets that replace the semicolon following `mod garden`
        - In the file `src/garden/rs`
        - In the file `src/garden/mod.rs`
 * Declaring submodules
    - In any file other than the crate root, you can declare `submodules`
    - If you declare `mod vegetables;` in src/garden.rs, the compiler will look for the submodule's code within the directory named for the parent module in these places:
        - Inline, directly following `mod vegetables` within curly brackets instead of semicolon
        - In the file `src/garden/vegetables.rs`
        - In the file `src/garden/vegetables/mod.rs`
 * Paths to code in modules
    - Once a module is part of your crate, you can refer to code in that module from anywhere else in the same crate, as long as the privacy rules allow, using the path to the code
    - For example, an `Asparagus` type in the garden vegetables module would be found at `crate::garden::vegetables::Asparagus`
 * Private vs. public
    - Code within a module is private from its parent modules by default
    - To make a module public, declare it with `pub mod` instead of `mod`
    - To make items within a public module public as well, use `pub` before their declarations
 * The `use` keyword
    - Within a scope, the `use` keyword creates shortcuts to items to reduce repetition of long paths
    - In any scope that can refer to `crate::garden::vegetables::Asparagus`, you can create a shortcut with `use crate::garden::vegetables::Asparagus;` and from then on you only need to write `Asparagus` to make use of that type in the scope
 
 ##### Paths for Referring to an Item in the Module Tree

 * A path can take two forms:
    - An `absolute path` is the full path starting from a crate root. for code from an extarnal crate, the absolute path begins with the crate name, and for code from the current crate, it starts with the literal `crate`
    - A `relative path` starts from  the current module and uses `self`, `super`, or an identifier in the current module
 * Both absolute and relative paths are followed by identifiers separated by double colors `::`
 

