// fn main() {
//     let num = 4; // Example number, change as needed
//     println!("Even number: {}", even_num(num));
// }

use std::io;

fn main() {
    // Prompt the user to enter a number
    println!("If no. is even then true else false!");

    println!("Enter a number: ");

    // Create a mutable variable to store the input
    let mut input = String::new();

    // Read the input from the user
    io::stdin().read_line(&mut input).unwrap();

    // Convert the input to an integer
    let num: i32 = input.trim().parse().unwrap();

    // Call the function to check if the number is even and print the result
    
    println!("Result: {}", even_num(num));
}

fn even_num(num: i32) -> bool {
    if num % 2 == 0 {
        return true;
    } else {
        return false;
    }
}
