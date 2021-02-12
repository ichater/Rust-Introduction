// Variables are immutable by default but can be changed with "mut"
// Rust is a block scoped language

pub fn run() {
    let name = "Izaak";
    let mut age = 32;

    println!("my name is {} and I am {} years old", name, age);
    age = 22;
    println!("my name is {} and I am {} years old", name, age);

    // Define constant, consts are strong typed
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // assign multiple vars
    let(my_name, my_age) = ("izaak", 32);
    println!("{} is {}",my_name, my_age );
}