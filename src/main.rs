use std::io;
use std::io::prelude::*;

fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    // allows cursor to stay at the end of the line, manual flush
    write!(stdout, "Press 'Enter' to exit. . .").unwrap();
    stdout.flush().unwrap();

    // read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
}

fn main() {
    
    println!("This program will calculate the 'n'th number in the Fibonacci sequence up to the 184th digit.");
    println!("Enter an integer 'n' between 0 and 184");

    // input nth number
    let mut input = String::new();

    io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
    
    let user_in = &input.trim();
    // panics if input is not an integer
    match user_in.parse::<i128>() {
        Ok(..) => print!(""),
        Err(e) => println!("Input was not an integer, ERROR: {}", e),
    };

    let n_fib = user_in.parse::<f64>().unwrap();
    
    // calculate nth number in Fibonacci sequence
    let phi_pos: f64 = (1.0 + (5_f64.sqrt()))/2.0;
    let phi_neg: f64 = (1.0 - (5_f64.sqrt()))/2.0;

    let fib = ( (phi_pos.powf(n_fib)) - (phi_neg.powf(n_fib)) ) / 5_f64.sqrt();
    // integer conversion
    let fib_int = fib as i128;
    let fib_str = fib_int.to_string();

    let mut vec = vec![];

    // if number is longer than 7 digits convert to scientific notation (ie - #.## x 10 ^ ##)
    if fib_str.len() > 7 {
        for c in fib_str.chars() {
            vec.push(c);
        } 

        print!("The {}th number in the Fibonacci sequence is: {}.", n_fib, vec[0]);
        // 6 sig figs
        for i in 1..6 {
            print!("{}", vec[i])
        }
        println!(" x 10^{}", (vec.len() - 1));        
    }
    else {
        println!("The {}th number in the Fibonacci sequence is: {}", n_fib, fib_str);
    }

    pause();
}