pub fn run() {
    greeting("Hello", "Brad");
    
    // Bind function values to variables
    let sum = add(5, 5);
    println!("Sum: {}", sum);
    println!("Sum: {}", add(5, 5)); // Same thing, we're just not using a middle variable

    // Closure
    let c: i32 = 10;
    let add_nums = |a: i32, b: i32| a + b + c;
    println!("Closure sum: {}", add_nums(3, 3));
}

fn greeting(greet: &str, name: &str) {
    println!("{} {}, nice to meet you !", greet, name);
}

// Function returning an in=32 integer
fn add(a: i32, b: i32) -> i32 {
    return a + b; // Or just a + b (no semi-colon)
}