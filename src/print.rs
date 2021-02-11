pub fn run(){
    //Print to console
    println!("Hello from print.rs file");

    println!("Number: {}", 1);
    // Basic Formatting
    println!("{} is from {}", "Izaak", "Australia");

    // Using index for place holders
    println!("{0} is from {1} and {0} likes to {2}", "Izaak", "Australia", "code");

    //Named Arguments
    println!("{name} likes to do {activiy}", name = "Izaak", activiy= "BJJ");

    //Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10,10,10)

    // Placeholder for debug trait
    println!("{:? }")
}