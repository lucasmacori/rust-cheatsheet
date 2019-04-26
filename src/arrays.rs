// Arrays are fixed list with same data types elements
// Vectors provide more flexibility than arrays

use std::mem;

pub fn run() {
    let mut numbers: [i32; 4] = [1, 2, 3, 5];

    // Re-assign a value
    numbers[2] = 20;

    println!("{:?}", numbers);

    // Get single val
    println!("First value of the array: {}", numbers[0]);

    // Get length of the array
    println!("Array length: {}", numbers.len());

    // Arrays are stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    // Get slice
    let slice: &[i32] = &numbers[1..3]; // Gets [2, 20] as a reference
    println!("Slice: {:?}", slice);
}