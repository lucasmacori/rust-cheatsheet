pub fn run() {
    // Primitive array
    let array1: [u8; 3] = [1, 2, 3];
    let array2: [u8; 3] = array1;

    println!("Values: {:?}", (array1, array2));

    /*
        With non-primitive, if you assign another variable to a piece of data,
        the first variable will no longer hold that value.
        You'll need to use a reference (&) to point to the ressource.
    */
    let vector1: Vec<u8> = vec![1, 2, 3];
    let vector2: &Vec<u8> = &vector1;
    
    println!("Values: {:?}", (&vector1, vector2));
}