# Chapter 2.0 - Programming a Guessing Game

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
    + Refer to [Chapter 1.3 - Hello, Cargo!](../ch1.3-Hello_Cargo/README.md) of the Rust Book for instructions to setup and use cargo

- Verify that cargo is installed
    ```bash
    cargo --version
    ```

### Contents
#### Initial Setup
- Create a project directory using cargo
    - Explanation
    ```bash
    cargo new guessing_game
    ```

- Change directory into project directory
    ```bash
    cd guessing_game
    ```

- Look at the generated 'Cargo.toml' configuration file
    ```toml
    [package]
    name = "guessing_game"
    version = "0.1.0"
    edition = "2021"
    # See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

    [dependencies]
    ```

- Look at the generated 'src/main.rs' source file
    ```rust
    fn main() {
        println!("Hello, world!");
    }
    ```

- Compile the source file
    ```bash
    cargo run
    ```

#### Processing a guess
- Edit the source file with the guessing game program logic
    - Explanation
        - The guessing game program will
            1. Ask for user input
            2. Process the user input
            3. Check that the user input is in the expected form
        - Source File Syntax structure/layout
            - `use std::io` : Import the library 'io' from the package 'std' which refers to the Rust standard library package, and io is a library/module within the standard library that handles Input-Output operations
                + Similar to C++'s `#include <iostream>` or C's `#include <cstdio.h>`
            + `println!(str)` : Print a new line as a macro
            - `let mut str_variable = String::new();` : Initialize a new string object and make it mutable 
                + in rust, variables are immutable (unmodifiable/constants) by default
            + `io::stdin()` : This function returns an instance of `std::io::Stdin`, which is a type that represents a handle to the standard input for your terminal 
            - `.read_line(&mut guess).expect("Failed to read line");` : Calls the `.read_line()` method on the standard input handle to get input from the user and 
                + store/append the value into the memory address pointing to the mutable variable 'guess', without overwriting its contents
            + `println!("You guessed: {guess}");` : Format string print the result variable
    - Notes
        - Operators
            + `=` : Set operator; Set the value of the Left-hand Side (LHS) to the value on the Right-hand Side (RHS)
            + `::` : Pointer/Reference to a memory address in a variable
            + `//` : Comments
            + `/* */` : Multi-line block comments
            + `&` : Reference-of Operator; assigning this to a variable basically returns the memory address of the pointer pointing to the variable, essentially 'referencing' and accessing the data of the variable without needing to copy that data into memory multiple times.
        - Keywords
            + `use [library]` : Import the specified library/class/function/attributes
            + `let variable_name = value;` : Declare and Initialize a new variable (variables on declaration are immutable by default in Rust)
            + `mut` : Set/Define a variable as mutable
        - Syntax/Structure
            + Importing libraries/modules: `use package::class::function|attribute;`
            + Format string: `{variable_here}`
            - Initializing a new variable
                + Default (Immutable): `let variable_name = value;`
                + Default a mutable variable: `let mut variable_name = value;`
                + Initializing a new String object: `let mut variable_name = String::new();`
            + Executing functions: `library::class|function().function();`
            + Throw exception catching: `library::function().functions().expect("your-exception-message-here");`
        - Packages
            + std : Rust standard library
        - Libraries/Modules
            - std
                - `::io` : Standard Input-Output operations library
        - Classes
            - std::io
                - `::stdin()` : Receive standard input (stdin) from the user via the stdin stream
                    - Return
                        - stdin : The standard input handle object which will allow us to handle user input
                            + Type: std::io::Stdin
        - Data Types/Structures
            - `enum` : Enumeration; A type that can be in one of multiple possible states, and each possible state is called a 'variant'
                - `.Result` : A Exception handling enumeration that holds the result status of functions and to encode error-handling information
                    - Variants
                        - Ok : Indicates that the operation was successful
                            + Value: The successfully generated value
                        - Err : Indicates that the operation failed
                            + Value: Contains information about how or why the operation failed
            + `String` : String type
            + `std::io::Stdin` : Data type that represents a handle to the standard input for your terminal
        - Functions
            - std
                - `println(str)` : Print a new line
                    - Parameter Signature/Header
                        + str : Print the specified message to the standard output
            - std::io::stdin()
                - `.read_line(buffer)` : Read the line received by the standard input stream and store the result in the specified buffer (memory container - i.e. the memory address pointing to a String variable)
                    + Type: Bytes
                    + Format: `&mut str_variable`
                    - Return
                        - Result : besides storing the value into the String container, the function will also return a Results enumeration (enum) value
                            + Type: enum.Result
            - enum.Result
                - `.expect(error_message)` : Function used for Exception handling (try catch) where when an exception is triggered, the program will crash and an error message will be printed
                    - Parameter Signature/Header
                        - error_message : Specify the error message to print when error has been caught
                            + Type: String
                    - Notes
                        - If you dont call `.expect(error_message)` after calling a function that returns an `enum.Result` type object
                            + The program will compile but you'll get a warning
                            - Rust warns that you havent used the `enum.Result` value returned from the function, 
                                + indicating that the program has not handled a possible error
            - String
                - `::new()` : Initialize a new String variable/object
                    - Return
                        - str_variable : Return the initialized String object instance
                            + Type: String
        - Attributes/Variables
    ```rust
    use std::io;

    fn main() {
        // Print a new line
        println!("Guess the number!");
        println!("Please input your guess.");

        // Initialize a new string object and make it mutable (in rust, variables are immutable by default)
        let mut guess = String::new();

        // Obtain Standard input from the user, read the line and store the value into the memory address pointing to the variable 'guess'
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Print result with format string
        println!("You guessed: {guess}");
    }
    ```

#### Generating a Secret Number
- Edit the generated 'Cargo.toml' configuration file
    - Add the 'rand' crate (aka dependencies)
        - Explanation
            - Add a new crate (aka "dependency packages") under the 'dependencies' section
                + by appending `crate-name = "version-number"` in a newline
        - Notes
            + In Rust, dependency packages are known as 'crates'
        ```toml
        [package]
        name = "guessing_game"
        version = "0.1.0"
        edition = "2021"
        # See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

        [dependencies]
        rand = "0.8.5"
        ```

- Edit the source file
    - Explanation
        + `use rand::Rng;` : Add the library 'rand::Rng'
        + `let secret_number = rand::thread_rng().gen_range(start..=end);` : Declare and initialize a secret number variable holding a randomized number generated from a range of numbers between minimum starting number and a maximum ending number
    - Notes
        - Packages
            - rand
        - Libraries/Modules
        - Classes
            - rand
                - `::thread_rng()` : Initialize a Random Number Generator object that is local to the current thread of execution and is seeded by the Operating System
        - Data Types/Structures
        - Functions
            - rand::thread_rng()
                - `.gen_range(start..=end)` : Generate a random number from a range of numbers between minimum starting number and a maximum ending number
                    - Parameters Signature/Header
                        - start : Specify the starting (minimum) number in the range of the random number generator pool
                            + Type: Integer
                        - end : Specify the ending (maximum) number in the range of the random number generator pool
                            + Type: Integer
                    - Return
                        - random_number : A Random number generated from a range of numbers between minimum starting number and a maximum ending number
                            + Type: Integer
        - Attributes/Variables
    ```rust
    use std::io;
    use rand::Rng;

    fn main() {
        // ...

        // Print a new line
        println!("Guess the number!");

        // Declare and initialize a secret number variable holding a randomized number generated 
        // from a range of numbers between minimum of 1 and a maximum of 100
        let secret_number = rand::thread_rng().gen_range(1..=100);

        println!("The secret number is {secret_number}");

        // ...

        println!("Please input your guess.");
    }
    ```

- Build the project
    ```bash
    cargo build
    ```

- Updating a crate to get a new version
    ```bash
    cargo update
    ```

#### Comparing the Guess to the secret number
- Edit the source file
    - Explanation
        + `use std::cmp::Ordering;` : Add the standard library 'std::cmp::Ordering'
        - `match guess.cmp(&secret_number) {}` : Compare the guess input value with the secret number value and use the result of the comparison in a Match-case (aka Switch-case)
            - `Ordering::Less` : guess is less than secret_number
            - `Ordering::More` : guess is more than secret_number
            - `Ordering::Equal` : guess is equals to secret_number
    - Notes
        - Operators
        - Keywords
            - `match <comparison-subject> { case_value => function(), ... }` : Match-case (aka switch-case) in rust
                - Synopsis/Syntax layout structure
                    ```rust
                    match <comparison> {
                        "result-case-1" => function_to_execute(arguments, ...),
                        "result-case-2" => function_to_execute(arguments, ...),
                        "result-case-3" => function_to_execute(arguments, ...),
                        "result-case-4" => function_to_execute(arguments, ...),
                    }
                    ```
        - Packages
            + std : Rust standard library
        - Libraries/Modules
            - std
                - `::cmp` : Standard comparison library
                - `::io` : Standard Input-Output operations library
        - Classes
            - std::io
                - `::stdin()` : Receive standard input (stdin) from the user via the stdin stream
                    - Return
                        - stdin : The standard input handle object which will allow us to handle user input
                            + Type: std::io::Stdin
        - Data Types/Structures
            - `enum` : Enumeration; A type that can be in one of multiple possible states, and each possible state is called a 'variant'
                - `.Result` : A Exception handling enumeration that holds the result status of functions and to encode error-handling information
                    - Variants
                        - Ok : Indicates that the operation was successful
                            + Value: The successfully generated value
                        - Err : Indicates that the operation failed
                            + Value: Contains information about how or why the operation failed
            + `String` : String type
            - `std::cmp::Ordering` : Enum that has the 3 outcomes that are possible when you compare 2 values - Less, Greater, Equal
                + Type: enum
                - Variants
                    + Less
                    + Greater
                    + Equal
            + `std::io::Stdin` : Data type that represents a handle to the standard input for your terminal
        - Functions
            - std
                - `println(str)` : Print a new line
                    - Parameter Signature/Header
                        + str : Print the specified message to the standard output
            - std::io::stdin()
                - `.read_line(buffer)` : Read the line received by the standard input stream and store the result in the specified buffer (memory container - i.e. the memory address pointing to a String variable)
                    + Type: Bytes
                    + Format: `&mut str_variable`
                    - Return
                        - Result : besides storing the value into the String container, the function will also return a Results enumeration (enum) value
                            + Type: enum.Result
            - enum.Result
                - `.expect(error_message)` : Function used for Exception handling (try catch) where when an exception is triggered, the program will crash and an error message will be printed
                    - Parameter Signature/Header
                        - error_message : Specify the error message to print when error has been caught
                            + Type: String
                    - Notes
                        - If you dont call `.expect(error_message)` after calling a function that returns an `enum.Result` type object
                            + The program will compile but you'll get a warning
                            - Rust warns that you havent used the `enum.Result` value returned from the function, 
                                + indicating that the program has not handled a possible error
            - String
                - `::new()` : Initialize a new String variable/object
                    - Return
                        - str_variable : Return the initialized String object instance
                            + Type: String
                - `.cmp(&variable)` : Compare the string value with another string value
                    - Return
                        - cmp_result: The comparison result - Less if the string is less than the target, More if the string is more than the target, Equal if the string is equals to the target
                            + Type: `std::cmp::Ordering`
        - Attributes/Variables
    ```rust
    use std::io;
    use std::cmp::Ordering;
    use rand::Rng;

    fn main() {
        // ...

        // Match-case (aka switch case) the guess input value with the secret number value
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => println("You win!"),
        }
    }
    ```

- Build the project
    ```bash
    cargo build
    ```

- Updating a crate to get a new version
    ```bash
    cargo update
    ```

## Wiki

### Tools
+ rustfmt : Automatic rust formatter/linter tool

## Resources

## References
+ [Rust Book - Chapter 1.1 - Installation](https://doc.rust-lang.org/book/ch01-01-installation.html)
+ [Rust Book - Chapter 1.2 - Hello, World!](https://doc.rust-lang.org/book/ch01-02-hello-world.html)
+ [Rust Book - Chapter 2.0 - Programming a Guessing Game](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html)
+ [Rustlang - Tools - Installation](https://www.rust-lang.org/tools/install)

## Remarks
