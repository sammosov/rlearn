fn main() {
    println!("{} days", 31);

//positional arguments
println!("{0}, this is {1}. {1}. this is {0}","Alice","Bob");
    
//As cannamed arguments
println!("{subject},{verb},{object}",
        object="tha lazy dog", 
        subject="the quick brown fox",
        verb="jumps over");

//Different formatting can be invoked by specifying the format character
println!("Base 10:                 {}", 60429);
println!("Base 2 (binary):       {:b}", 69429);
println!("Base 8 (octal):        {:o}", 69429);
println!("Base 16 (hexadecimal): {:x}", 69429);
    
// You can rigt justify text wiyj a specified width
println!("{number:>5}", number=1);

// You can pad numbers with extra zeroes
println!("{number:0>5}", number=1);
// and reverse
println!("{number:0<5}", number=1);

//You can use nmaed arguments in the format speciffied by appendig a $
println!("{number:0>width$}", number=1, width=5);

//Rust even checks to make sure the correct number of arguments ate used
println!("My name is {0}, {1} {0}", "Bond", "James");

// Only types that implement fmt::Disapley can be formatted with {}. 
// User defined types do not implement fmt::Disaplay by default
#[allow(dead_code)] //disable dead_code which warn egainst unused module
//stuct Structure(is32);
//println!("This struct '{}' wont print...", Structure(3));

// for Rust 1.50 anf abobe, yo can diretly capture the argument from a surrounding variables
let number: f64 = 1.0;
let width: usize = 5; 
println!("{number:>width$}");

//Add a println! macro call that prints: Pi is roughly 3.142 by controlling the number of decimal places shown. 
// For the purposes of this exercise, use let pi = 3.141592 as an estimate for pi
//let pi_number: f64 = 3.141592;
//println!("Pi number roughtly = {}, '{pi_number:.*}'", 3, pi_number);
//println!("{}, '{pi_number:.*}' has a 3 fractional digits","Hello",3,pi_number);
}
