use functions::sum;
use variables::{arguments, lp, var};
mod functions;
mod variables;
use std;

fn main() {
    //    arguments();
    //    let x = 23;
    //    println!("The value of x {}", x);

    let mut arg = std::env::args();
    let firstarg = arg.next().unwrap();
    let key = arg.next().unwrap();
    let value = arg.next().unwrap();

    println!("The key is {}", key);
    println!("The value is {}", value)
    //  somestuff();
}

fn somestuff() {
    let name = "rakib";
    println!("{}", name);
}
