fn main() {
    println!("Variables and Mutability in Rust");
    println!("---------------------------------");
    let_keyword();
    println!("---------------------------------");
    const_keyword();
    println!("---------------------------------");
    shadowing();
}

fn let_keyword() {
    let mut age = 22;
    println!("The Value of age is: {}", age);

    age = 23;
    println!("The Value of age is: {}", age);
    // The value of age can be changed because it is mutable
    // If we try to change the value of age without using mut keyword, it will give an error
    // Because by default variables are immutable in Rust
    // To make a variable mutable, we need to use mut keyword
}

fn const_keyword() {
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Three Hours in Seconds: {}", THREE_HOURS_IN_SECONDS);

    let one_hour = THREE_HOURS_IN_SECONDS / 3;
    println!("One Hour in Seconds: {}", one_hour);

    // Constants are always immutable
    // Constants are always declared using const keyword
    // Constants are always in uppercase
    // Constants can be declared in any scope, including the global scope
    // Constants can only be set to a constant expression, not the result of a function call or any other value that could only be computed at runtime
    // Constants are valid for the entire time a program runs, within the scope they are declared in
    // Constants are not allowed to use mut keyword
    // Constants are not allowed to use shadowing
    // Constants are not allowed to use static keyword
    // Constants are not allowed to use let keyword
}

fn shadowing() {
    let apple = 10;
    println!("apple: {apple}"); // The Value of apple is: 5

    let apple = apple + 10;
    println!("apple: {apple}"); // The Value of apple is: 15

    let apple = true;
    println!("Is Apples Are Available: {apple}"); // Is Apples Are Available: true

    // Shadowing is different from marking a variable as mutable
    // If we forget to write let keyword, it will give an error because we are trying to reassign a value to a variable which is immutable
    // that's why we need to use let keyword to shadow a variable
    // Shadowing is useful when we want to change the type of a variable but keep the same name
    // Shadowing is also useful when we want to change the value of a variable but keep the same name
    // Shadowing is also useful when we want to change the value of a variable but keep the same name and type
    // Shadowing is also useful when we want to change the value of a variable but keep the same name and type and make it mutable
    // for eg. let mut x = 5; let x = x + 1; // This will give an error because we are trying to shadow a mutable variable
    // another eg. let x = 5; let x = x + 1; // This will work because we are trying to shadow an immutable variable

    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    // The value of x in the inner scope is: 12
    // The value of x is: 6
    // The value of x in the inner scope is 12 because we are shadowing the value of x in the inner scope
    // The value of x is 6 because we are shadowing the value of x in the outer scope
}
