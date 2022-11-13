use std::io;        // Brings input/outpu lib into scope
use std::fs;        // Brings file lib into scope

fn main() {
    println!("This is the voltage-divider calculator.");
    let filename: &str = "graphic.txt";

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    
    println!("\n{}", contents);
    println!();
    println!("Insert R1");
    
    let mut r1 = String::new();
    let mut r2 = String::new();
    let mut vi = String::new();

    io::stdin()
        .read_line(&mut r1)
        .expect("Failed to read line.");
    
    println!("Insert R2");
    
    io::stdin()
        .read_line(&mut r2)
        .expect("Failed to read line.");

    println!("Insert Vi");

    io::stdin()
        .read_line(&mut vi)
        .expect("Failed to read line.");

    println!("Vi: {} Volt", vi);
    println!("R1: {} Ohm and R2: {} Ohm", r1, r2);      // new line after {R1} ???

    let r1: f32 = r1.trim().parse().expect("Please type a number.");
    let r2: f32 = r2.trim().parse().expect("Please type a number.");
    let vi: f32 = vi.trim().parse().expect("Please type a number.");

    let vo: f32 = r2 * vi / (r1 + r2);
                 

    println!("Vo: {} Volt", vo);
}
