# Data Types in Rust

This project explores **Data Types** in the Rust programming language. Rust has two main categories of data types: **Scalar Types** and **Compound Types**. This code demonstrates each type with examples, providing a hands-on way to understand Rust's type system.

---

## Concepts Covered

### 1. Scalar Types

Scalar types represent a single value. Rust has four primary scalar types:
- **Integers**
- **Floating Point**
- **Boolean**
- **Characters**

#### Integer Types

Integers in Rust come in various sizes and can be signed (`i`) or unsigned (`u`). Common types include `i32`, `u8`, and `i64`.

Example:
```rust
fn integers() {
    let x: i32 = 42;
    let y: u8 = 255;
    let z: i64 = -100000;

    println!("i32: {}", x);
    println!("u8: {}", y);
    println!("i64: {}", z);
}
```

#### Floating Point Types
Rust provides `f32` and `f64` types for floating-point numbers, with `f64` offering more precision. 

Example:
```rust
fn floating_point() {
    let x: f32 = 3.1234567;
    let y: f64 = 3.12345678923456;

    println!("f32: {}", x);
    println!("f64: {}", y);
}
```

#### Numeric Operations
Rust supports basic arithmetic operations like `addition`, `subtraction`, `multiplication`, `division`, and `remainder`.

Example:
```rust
fn numeric_operations() {
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let remainder = 43 % 5;

    println!("Sum: {}", sum);
    println!("Difference: {}", difference);
    println!("Product: {}", product);
    println!("Quotient: {}", quotient);
    println!("Remainder: {}", remainder);
}
```

#### Boolean Type

Boolean types represent logical values and can be either `true` or `false`.

Example:
```rust
fn boolean() {
    let t = true;
    let f: bool = false;

    println!("t: {}", t);
    println!("f: {}", f);
}
```

#### Character Type

Rust's `char` type represents a single character. Characters are Unicode, allowing for emojis and other symbols.

Example:
```rust
fn characters() {
    let c = 'A';
    let smiley = '😊';

    println!("c: {}", c);
    println!("smiley: {}", smiley);
}
```
---

### 2. Compound Types

Compound types group multiple values into one type. Rust has two primary compound types: 
- **Tuples**
- **Arrays**
  
#### Tuples

Tuples are collections of values of different types. They have a fixed length and are declared using parentheses. 

Example:
```rust
fn tuples() {
    let tuple: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tuple;

    println!("x: {}", x);
    println!("y: {}", y);
    println!("z: {}", z);
    println!("Accessing with tuple.0: {}", tuple.0);
}
```     

#### Arrays

An array in Rust is a collection of values of the same type. Arrays have a fixed size and are useful when you know the number of elements in advance.

Example:
```rust
fn array() {
    let a = [1, 2, 3, 4, 5];
    let b: [i32; 5] = [1, 2, 3, 4, 5];
    let c = [3; 5]; // array of five 3s

    println!("a: {:?}", a);
    println!("b: {:?}", b);
    println!("c: {:?}", c);
    println!("Accessing a[0]: {}", a[0]);
}
```

#### output
```bash
Data Types in Rust
------------------
//////////////////
   SCALAR TYPES   
//////////////////
Integers
i32: 42
u8: 255
i64: -100000
------------------
Floating Point
f32: 3.1234567
f64: 3.12345678923456
------------------
Numeric Operations
Sum: 15
Difference: 91.2
Product: 120
Quotient: 1.7608695652173911
Remainder: 3
------------------
Boolean
t: true
f: false
------------------
Characters
c: A
smiley: 😊
------------------

//////////////////
  COMPOUND TYPES  
//////////////////
Tuples
x: 500
y: 6.4
z: 1
Accessing with tuple.0: 500
------------------
Arrays
a: [1, 2, 3, 4, 5]
b: [1, 2, 3, 4, 5]
c: [3, 3, 3, 3, 3]
Accessing a[0]: 1
------------------
```
---

## Running the Code

Note: Make sure you have Rust installed on your system. If not, you can install it from [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).

1. Clone the repository:
   ```
   git clone https://github.com/shoaib-eth/learn-rust-with-shoaib.eth.git
   ```
2. Navigate to the project directory:
   ```
   cd learn-rust-with-shoaib.eth/02_data_types
   ```
3. Run the code using the following command:
   ```
   cargo run
   ```
4. The output will be displayed in the terminal.
5. You can modify the code in `src/main.rs` to explore different scenarios and test the concepts covered in this project.
6. Enjoy exploring Rust variables and mutability!
7. Happy Coding! 🚀
---