pub fn run() {
    let mut count: u8 = 0;

    // Standart loop
    loop {
        count += 1;
        println!("Number: {}", count);

        if count == 20 {
            break;
        }
    }

    // While loop (FizzBuzz)
    count = 1;
    while count <= 100 {
        if count % 15 == 0 {
            println!("FizzBuzz");
        } else if count % 3 == 0 {
            println!("Buzz");
        } else {
            println!("{}", count)
        }
        count += 1;
    }

    // For range (FizzBuzz again)
    for x in 1..100 {
        if x % 15 == 0 {
            println!("FizzBuzz");
        } else if x % 3 == 0 {
            println!("Buzz");
        } else {
            println!("{}", x)
        }
    }
}