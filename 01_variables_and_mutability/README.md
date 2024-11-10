# Variables and Mutability in Rust

This project explores the concepts of **Variables** and **Mutability** in the Rust programming language. Through this code, we understand how Rust handles variables with `let`, `const`, and `shadowing`. Each concept is explained below, with examples to illustrate how they work in Rust.

---

## Concepts Covered

### 1. `let` Keyword

The `let` keyword in Rust is used to create variables. By default, variables in Rust are **immutable**, meaning their values cannot be changed after they are set. However, if we want to make a variable mutable, we can use the `mut` keyword along with `let`.

#### Example:
```rust
fn let_keyword() {
    let mut age = 22;
    println!("The Value of age is: {}", age);

    age = 23; // Changing the value of age
    println!("The Value of age is: {}", age);
}
```

Here, `age` is declared as a mutable variable using `let mut`, allowing us to change its value from `22` to `23`. If `mut` was not used, Rust would give an error when trying to modify `age`.

#### 2. `const` Keyword

Constants in Rust are created using the `const` keyword. Unlike variables created with `let`, constants are always immutable, and they are typically written in uppercase letters. Constants must be set to values that are known at compile time (constant expressions), and they remain valid throughout the program.

#### Example:
```rust
fn const_keyword() {
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Three Hours in Seconds: {}", THREE_HOURS_IN_SECONDS);

    let one_hour = THREE_HOURS_IN_SECONDS / 3;
    println!("One Hour in Seconds: {}", one_hour);
}
```

Here, `THREE_HOURS_IN_SECONDS` is a constant representing the number of seconds in three hours. Constants:

- Are always immutable (cannot use `mut` with `const`)
- Cannot use `shadowing`
- Cannot use values computed at runtime

#### 3. Shadowing

Shadowing allows us to reassign a variable with the same name to a new value or even a new type, using the `let` keyword again. Shadowing is different from mutability as it allows us to redefine a variable in the same scope or a nested scope without making it mutable.

#### Example:
```rust
fn shadowing() {
    let apple = 10;
    println!("apple: {apple}");

    let apple = apple + 10;
    println!("apple: {apple}");

    let apple = true;
    println!("Is Apples Are Available: {apple}");

    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}
```

In this example:

- `apple` is redefined with different types and values multiple times using `let`, demonstrating shadowing.
- `x` is redefined in an inner scope, temporarily changing its value, but outside the inner scope, `x` retains its value from the outer scope.

### Output Example
When running this code, the output would be:

```
Variables and Mutability in Rust
---------------------------------
The Value of age is: 22
The Value of age is: 23
---------------------------------
Three Hours in Seconds: 10800
One Hour in Seconds: 3600
---------------------------------
apple: 10
apple: 20
Is Apples Are Available: true
The value of x in the inner scope is: 12
The value of x is: 6
```

### conscusion
This project demonstrates how Rust handles variables and mutability using `let`, `const`, and `shadowing`. By understanding these concepts, we can effectively manage variables in Rust, ensuring that our code is both safe and efficient.

--- 

## How to Run the Code

Note: Make sure you have Rust installed on your system. If not, you can install it from [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).

1. Clone the repository:
   ```
   git clone https://github.com/shoaib-eth/learn-rust-with-shoaib.eth.git
   ```
2. Navigate to the project directory:
   ```
   cd learn-rust-with-shoaib.eth/01_variables_and_mutability
   ```
3. Run the code using the following command:
   ```
   cargo run
   ```
4. The output will be displayed in the terminal.
5. You can modify the code in `src/main.rs` to explore different scenarios and test the concepts covered in this project.
6. Enjoy exploring Rust variables and mutability!
7. Happy Coding!

---