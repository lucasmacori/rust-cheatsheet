// Custom data types

// Traditional struct
/*struct Color {
    red: u8,
    green: u8,
    blue: u8
}*/

// Tuple struct
//struct Color(u8, u8, u8);
struct Person {
    first_name: String,
    last_name: String
}

impl Person {
    // Construct a person
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string()
            // "to_string()" is needed because we have set the types of the struct as "String".
            // Which is different from &str
        }
    }
}

pub fn run() {
    // Traditional struct
    /*let mut c = Color {
        red: 255,
        green: 0,
        blue: 0
    };

    c.red = 200;
    
    println!("Color: {} {} {}", c.red, c.green, c.blue);*/

    // Tuple struct
    /*let mut c = Color(255, 0, 0);

    c.0 = 200;

    println!("Color: {} {} {}", c.0, c.1, c.2);*/

    let mut p = Person::new("John", "Doe");
    println!("Person: {} {}", p.first_name, p.last_name);
}