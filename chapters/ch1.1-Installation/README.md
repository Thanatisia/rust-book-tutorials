# Chapter 1.1 - Installation

## Setup
### Dependencies
+ curl: For installing rustup
+ cargo : Rust compiler
+ rustc : Rust compiler
+ rustdoc : Rust documentations
+ rustup : Rust updater

### Pre-Requisites
- Setup rustup
    - Notes
        - for a full write-up to setup rustup, cargo, rustc and rustdoc
            + Please refer to [rustup installation documentations](https://github.com/Thanatisia/SharedSpace/blob/main/Docs/Programming/Languages/Rust/Guides/Rust%20Compiler/installation-methods.md)
    - Linux
        - Using the rustup shellscript
            - Download the rustup install script
                ```bash
                curl --proto '=https' --tlsv1.2 https://sh.rustup.rs >> rustup.sh
                ```
            - (Optional - Recommended) Read the source code
                ```bash
                cat rustup.sh
                ```
            - Change permission modifier of the script to executable
                ```bash
                chmod u+x rustup.sh
                ```
            - Execute installation script
                + Make changes if necessary
                ```bash
                sh rustup.sh
                ```
    - Windows
        - Install the rust installer 'rustup-init.exe'
            + Go to https://www.rust-lang.org/tools/install
            - Download the installer 'rustup-init.exe'
                - 32-bit
                    ```bash
                    wget https://static.rust-lang.org/rustup/dist/i686-pc-windows-msvc/rustup-init.exe
                    ```
                - 64-bit
                    ```bash
                    wget https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe
                    ```
            + Begin Installation

- Verify rust is installed
    ```bash
    rustc --version
    ```

- (Optional) Set the rust working environment into your system PATH environment variable
    - Linux
        - The default rust installation root directory is '$HOME/.cargo'
            + Binary/Executables directory: '$HOME/.cargo/bin'
            + Virtual environment: $HOME/.cargo/env'
        ```bash
        export PATH+=":$HOME/.cargo/bin:"
        ```
    - Windows
        ```bash
        SET PATH="%PATH%;\path\to\.cargo\bin;"
        ```

### Post-Installation
- Update Rust
    - Using rustup
        ```bash
        rustup update
        ```

- Uninstall Rust
    - Using rustup
        ```bash
        rustup self uninstall
        ```

- Open a local copy of the rust documentation in your browser
    - Explanation
        + The installation of Rust also includes a local copy of the documentation so that you can read it offline.
    ```bash
    rustup doc
    ```

## Resources

## References
+ [Rust Book - Chapter 1.1 - installation](https://doc.rust-lang.org/book/ch01-01-installation.html)
+ [Rustlang - Tools - Installation](https://www.rust-lang.org/tools/install)

## Remarks

