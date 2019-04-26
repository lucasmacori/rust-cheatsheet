// Primitive str = immutable fixed-length string somewhre in memory
// String = Growable, heap-allocated data structure
// Use when you need to modify or own string data

pub fn run() {
    let mut hello = String::from("Hello ");

    // Get length
    println!("Length: {}", hello.len());

    // Push
    hello.push('W'); // only one char
    hello.push_str("orld"); // Now using a string

    // Capacity in bytes
    println!("Capacity: {}", hello.capacity());

    // Check if empty
    println!("Capacity: {}", hello.is_empty());

    // Contains
    println!("Contains 'World': {}", hello.contains("World"));

    // Replace
    println!("Replace: {}", hello.replace("World", "There"));

    // Loop through string by whitespace
    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    // Create a string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    // Assertion testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());

    println!("{}", s);

    println!("{}", hello);
}