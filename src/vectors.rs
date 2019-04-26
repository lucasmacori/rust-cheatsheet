// Vectors are rezisable arrays

use std::mem;

pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 5];

    // Re-assign a value
    numbers[2] = 20;

    // Add a value to the vector
    numbers.push(6);

    // Pop the last value
    numbers.pop();

    println!("{:?}", numbers);

    // Get single val
    println!("First value of the vector: {}", numbers[0]);

    // Get length of the vector
    println!("Vector length: {}", numbers.len());

    // Vectors are stack allocated
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

    // Get slice
    let slice: &[i32] = &numbers[1..3]; // Gets [2, 20] as a reference
    println!("Slice: {:?}", slice);

    // Loop though vector values
    for x in numbers.iter() {
        println!("{}", x);
    }

    // Loop and mutate values
    for x in numbers.iter_mut() {
        *x *=2;
    }

    println!("Numbers Vec: {:?}", numbers);
}