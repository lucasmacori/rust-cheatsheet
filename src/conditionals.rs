pub fn run() {
    let age: u8 = 16;
    let check_id: bool = false;
    let knows_person_of_age: bool = true;

    // if / else
    if (age >= 18 && check_id) || knows_person_of_age {
        println!("Bartender: What would you like to drink ?");
    } else if age < 18 && check_id {
        println!("Bartender: Sorry, you have to leave");
    } else {
        println!("Bartender: I'll need to see your ID");
    }

    // Sharthand if
    let is_of_age: bool = if age >= 21 { true } else { false };
    println!("Is of age: {}", is_of_age);
}