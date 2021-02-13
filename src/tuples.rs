//Group of values (can be different types), max 12 values.

pub fn run(){
    let person: (&str, &str, i8) = ("Izaak", "Footscray", 32);
    
    println!("{} is from {} and my age is {}", person.0, person.1, person.2)
}