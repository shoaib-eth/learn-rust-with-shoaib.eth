fn main() {
    println!("Data Types in Rust");
    println!("------------------");
    println!("//////////////////");
    println!("   SCALAR TYPES   ");
    println!("//////////////////");
    println!("Integers");
    integers();
    println!("------------------");
    println!("Floating Point");
    floating_point();
    println!("------------------");
    println!("Numeric Operations");
    numeric_operations();
    println!("------------------");
    println!("Boolean");
    boolean();
    println!("------------------");
    println!("Characters");
    characters();
    println!("------------------\n");
    println!("//////////////////");
    println!("  COMPOUND TYPES  ");
    println!("//////////////////");
    println!("Tuples");
    tuples();
    println!("------------------");
    println!("Arrays");
    array();
    println!("------------------");
}

////////////////////
/// SCALAR TYPES ///
////////////////////
fn integers() {
    let x: i32 = 42;
    let y: u8 = 255;
    let z: i64 = -100000;

    println!("i32: {}", x);
    println!("u8: {}", y);
    println!("i64: {}", z);
}

fn floating_point() {
    let x: f32 = 3.12345678923456;
    let y: f64 = 3.12345678923456;

    println!("f32: {}", x);
    println!("f64: {}", y);
}

fn numeric_operations() {
    // addition
    let sum = 5 + 10;
    println!("5 + 10 = {}", sum);

    // subtraction
    let difference = 95.5 - 4.3;
    println!("95.5 - 4.3 = {}", difference);

    // multiplication
    let product = 4 * 30;
    println!("4 * 30 = {}", product);

    // division
    let quotient = 56.7 / 32.2;
    println!("56.7 / 32.2 = {}", quotient);

    let truncated = -5 / 3;
    println!("-5 / 3 = {}", truncated);

    // remainder
    let remainder = 43 % 5;
    println!("43 % 5 = {}", remainder);
}

fn boolean() {
    let t = true;
    let f: bool = false;

    println!("t: {}", t);
    println!("f: {}", f);
}

fn characters() {
    let c = 'A';
    let z = 'Z';
    let smiley = '😊';

    println!("c: {}", c);
    println!("z: {}", z);
    println!("smiley: {}", smiley);
}

//////////////////////
/// COMPOUND TYPES ///
//////////////////////
fn tuples() {
    let tuple: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tuple;

    println!("x: {}", x);
    println!("y: {}", y);
    println!("z: {}", z);

    println!("tuple.0: {}", tuple.0);
    println!("tuple.1: {}", tuple.1);
    println!("tuple.2: {}", tuple.2);
}

fn array() {
    let a = [1, 2, 3, 4, 5];
    let b: [i32; 5] = [1, 2, 3, 4, 5];
    let c = [3; 5];

    println!("a: {:?}", a); // a: [1, 2, 3, 4, 5]
    println!("b: {:?}", b); // b: [1, 2, 3, 4, 5]
    println!("c: {:?}", c); // c: [3, 3, 3, 3, 3]

    println!("a[0]: {}", a[0]);
    println!("b[3]: {}", a[3]);
    println!("c[4]: {}", a[4]);
}
