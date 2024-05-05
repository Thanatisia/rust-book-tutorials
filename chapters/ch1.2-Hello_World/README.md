# Chapter 1.2 - Hello, World!

## Setup
### Dependencies
+ curl: For installing rustup
+ cargo : Rust compiler
+ rustc : Rust compiler
+ rustdoc : Rust documentations
+ rustup : Rust updater

### Pre-Requisites
- Setup Rust
    + Refer to [Chapter 1.1 - Installation](../ch1.1-Installation/README.md) of the Rust Book for instructions to install rustc

### Contents
- Create a project directory
    ```bash
    mkdir -pv projects
    ```

- Change directory into project directory
    ```bash
    cd projects
    ```

- Create a new source file 'main.rs'
    - Linux
        ```bash
        touch main.rs
        ```
    - Windows
        ```bash
        echo "" >> main.rs
        ```

- Open 'main.rs'
    - Create the main entry point function
        - Explanation
            - `fn main()` : Declare a new Function 'main' with no parameter signature/headers
                - Statements
                    - `println(msg);` : Print the specified message in a new line
                        - Equivalent to
                            + 'print(msg)' in python
                            + 'cout << msg' in C++
                            + 'printf("%s", msg)' in C
                            + 'System.Out.Println()' in C#
        - Notes
            - Anatomy of a rust program
                - Rust style is to indent with 4 spaces, not a tab
                - The '!' operator identifies a Rust macro
                    - i.e.
                        + `println()` calls a function
                        + `println!()` calls a macro
                - Rust ends a line of code with a semicolon (';') operator
                    + This indicates that the expression/statement is over
        ```rust
        fn main() {
            println!("Hello World!");
        }
        ```
    - Compile the source file into an executable
        - This works similar to gcc or clang for C/C++
            + rustc is a compiler that takes the source codes and compile them into a binary executable
        ```bash
        rustc main.rs
        ```
    - List/View the executables
        - Linux
            ```bash
            ls -lha
            ```
        - Windows
            ```bash
            dir --all
            ```
        - Powershell
            ```powershell
            dir /B %= the /B option says to only show the file names =%
            ```
    - Execute the executable
        - Linux
            ```bash
            ./main
            ```
        - Windows
            ```bash
            .\main.exe
            ```

## Wiki

### Tools
+ rustfmt : Automatic rust formatter/linter tool

## Resources

## References
+ [Rust Book - Chapter 1.1 - Installation](https://doc.rust-lang.org/book/ch01-01-installation.html)
+ [Rust Book - Chapter 1.2 - Hello, World!](https://doc.rust-lang.org/book/ch01-02-hello-world.html)
+ [Rustlang - Tools - Installation](https://www.rust-lang.org/tools/install)

## Remarks

