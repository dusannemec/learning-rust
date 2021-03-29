// Primitive str = Immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure - Use when you need to modify or own the string data

pub fn run() {
    let hello = "Hello";
    let mut mutable_hello = String::from("Hello Mutable");

    println!("{}", mutable_hello);

    // Get length
    println!("Length: {}", hello.len());

    // Push char
    mutable_hello.push('W');

    // Push string
    mutable_hello.push_str("orld!");

    // Capacity in bytes
    println!("Capacity: {}", mutable_hello.capacity());

    // Check if empty
    println!("Is Empty: {}", hello.is_empty());

    // Cointains
    println!("Contians 'World': {}", mutable_hello.contains("World"));

    // Replace
    println!("Replace: {}", mutable_hello.replace("MutableWorld", "There"));

    // Loop through string by whitespace
    for word in mutable_hello.split_whitespace() {
        println!("{}", word);
    }

    println!("{}", hello);
    println!("{}", mutable_hello);

    // Create string with capacity
    let mut s = String::with_capacity(10);

    s.push('a');
    s.push('b');

    // Assertion testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());

    println!("{}", s);

}