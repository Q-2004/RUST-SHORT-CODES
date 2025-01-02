fn main() {
    let x = 4;  // Example input (you can change this number to any other integer)
    println!("{}", fib(x));
}

fn fib(num: i32) -> i32 {
    let mut first = 0;
    let mut second = 1;

    if num == 0 {
        return first;  // Return 0 for num = 0
    }
    if num == 1 {
        return second;  // Return 1 for num = 1
    }

    // Fibonacci calculation for numbers greater than 1
    for _ in 2..=num {  // Start from 2 and go up to num (inclusive)
        let temp = second;
        second = second + first;  // The next Fibonacci number
        first = temp;  // Update the first to the previous second
    }

    second  // After the loop, 'second' will hold the Fibonacci number
}
