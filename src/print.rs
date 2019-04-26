pub fn run() {
    // Print to console
    println!("Hello from th print.rs file");

    // Basic formating
    println!("{} is from {}", "Brad", "Mass");

    // Positional arguments formating
    println!("{0} is from {1} and {0} likes to {2}", "Brad", "Mass", "code");

    // Names arguments formating
    println!("{name} likes to play {activity}", name = "John", activity = "Basketball");

    // Placeholder traits formating
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    // Placeholder for debug trait
    println!("{:?}", (12, true, "hello"));

    // Basic math
    println!("10 + 10 = {}", 10 + 10);
}