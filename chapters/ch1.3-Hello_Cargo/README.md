# Chapter 1.3 - Hello, Cargo!

## Information
### Description
- Cargo is Rust’s build system and package manager.
    + Most Rustaceans use this tool to manage their Rust projects because Cargo handles a lot of tasks for you,
        - such as
            + building your code,
            + downloading the libraries your code depends on,
            + and building those libraries. (We call the libraries that your code needs dependencies.)

- The simplest Rust programs, like the one we’ve written so far, don’t have any dependencies.
    - If we had built the “Hello, world!” project with Cargo,
        + it would only use the part of Cargo that handles building your code.

- As you write more complex Rust programs,
    + you’ll add dependencies,
    + and if you start a project using Cargo, adding dependencies will be much easier to do.

## Setup
### Dependencies
+ curl: For installing rustup
+ cargo : Rust compiler
+ rustc : Rust compiler
+ rustdoc : Rust documentations
+ rustup : Rust updater

### Pre-Requisites
- Setup cargo
    + Refer to [Chapter 1.1 - Installation](../ch1.1-Installation/README.md) of the Rust Book for instructions to install rustc

- Verify that cargo is installed
    ```bash
    cargo --version
    ```

### Contents
- Create a project directory using cargo
    - Explanation
        - Cargo will create a new directory, and within the directory
            - Initialize a new git repository, along with a `.gitignore` file which contains all the files/folders you want git to ignore
            - Generate 2 files and 1 directory
                + `Cargo.toml` file : this is the primary cargo project TOML configuration file
                - `src/` directory : For holding all your source codes
                    + `main.rs` : The main entry point rust source file
    - Notes
        - Git files wont be generated if you run `cargo new` within an existing Git repository
            + You can override this behavior by using `cargo new --vcs=[version-control-system (i.e. git)]`, which will let you specify a custom version control system to use
    ```bash
    cargo new [project-name]
    ```

- Change directory into project directory
    ```bash
    cd [project-name]
    ```

- Edit `Cargo.toml` with a text editor
    - Explanation
        - Cargo configuration and customization settings key-values
            - Header Keys
                - package : This section configures the project's package specifications
                    - key-value settings
                        - name : Specify the name of the package; (Recommended) name this the same thing as your project workspace root directory for clarity
                            + Type: String
                        - version : Specify the version of the package; Update this every time you change version (similar to pyproject.toml)
                            + Type: String
                        - edition : Specify the edition of rust to use
                            + Type: String
                - dependencies : This section configures the project's dependencies and required packages
                    - key-value settings
                        - 
    - Notes
        + Please refer to [Rustlang - Documentations - cargo - reference - manifest](https://doc.rust-lang.org/cargo/reference/manifest.html) for more keys and their definitions
    ```toml
[package]
name = "hello_cargo"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
    ```

- Edit 'src/main.rs' file
    ```rust
    fn main() {
        println!("Hello, world!");
    }
    ```

- Build cargo project
    - Explanation
        - Cargo will create an executable file in 'target/debug/[project-name]' (Linux) or 'target\debug\[project-name]' (Windows) instead of your current working directory.
            - Because the default build is a debug build, Cargo puts the binary in a directory named 'debug' 
                + Structure Layout: `target/[build-type]/[project-name]`
    - Notes
        - Running `cargo build` for the first time also causes Cargo to create a new file at the top level: Cargo.lock
            + This file keeps track of the exact versions of dependencies in your project
    ```bash
    cargo build
    ```

- Build cargo project for release
    - Explanation
        - Cargo will compile it with optimizations
            + and create an executable in 'target/release' instead of 'target/debug'
        - The optimizations will make your rust code run faster, 
            + but turning them on lengthens the time it takes for your program to compile
            - That is why there are 2 different profiles
                + one for development
                + one for building the final program you'll give to a user that wont be rebuilt repeatedly and that will run as fast as possible
    - Notes
        + If you are benchmarking your code's running time, use `cargo build --release` and benchmark with the executable in 'target/release'
    ```bash
    cargo build --release
    ```

- Run the executable
    - Linux
        ```bash
        ./target/[build-type]/[executable-name]
        ```
    - Windows
        ```bash
        .\target\[build-type]\[executable-name].exe
        ```

- Compile and run
    - Explanation
        + This will compile the code and then run the resulting executable all in one command
        - Cargo will figure out if the files had changes
            + If the files had not been changed, it doesnt rebuild the source code and just runs the binary
    - Notes
        + Using `cargo run` is more convenient than having to run `cargo build` and then use the whole path to the binary
    ```bash
    cargo run
    ```

- Check the code to make sure it compiles but dont compile
    - Explanation
        - This command will quickly check your code to make sure it compiles
            + But this will not produce an executable
    - Notes
        + This is useful as it skips the step of producing an executable
    ```bash
    cargo check
    ```

### Summary
- Cargo operational workflow
    - Create a new project
        ```bash
        cargo new [project-name]
        ```
    - Change directory into project
        ```bash
        cd [project-name]
        ```
    - Development
        - Build a project without producing a binary to check for errors
            ```bash
            cargo check
            ```
        - Build the project
            ```bash
            cargo build
            ```
        - Run the built executable binary
            - Linux
                ```bash
                ./target/[build-type]/[executable-name]
                ```
            - Windows
                ```bash
                .\target\[build-type]\[executable-name].exe
                ```
        - Build and run the project
            ```bash
            cargo run
            ```
    - Deployment/Production
        - Build for release
            ```bash
            cargo build --release
            ```

## Wiki

### Tools
+ rustfmt : Automatic rust formatter/linter tool

## Resources

## References
+ [Rust Book - Chapter 1.1 - Installation](https://doc.rust-lang.org/book/ch01-01-installation.html)
+ [Rust Book - Chapter 1.3 - Hello, Cargo!](https://doc.rust-lang.org/book/ch01-03-hello-cargo.html)
+ [Rustlang - Tools - Installation](https://www.rust-lang.org/tools/install)
+ [Rustlang - Documentations - Cargo](https://doc.rust-lang.org/cargo/)
+ [Rustlang - Documentations - cargo - reference - manifest](https://doc.rust-lang.org/cargo/reference/manifest.html) for more keys and their definitions

## Remarks

