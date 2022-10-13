use functions::sum;
use variables::{arguments, lp, var};
mod functions;
mod variables;

fn main() {
    arguments();
    let x = 23;
    println!("The value of x {}", x);

    somestuff();
}

fn somestuff() {
    let name = "rakib";
    println!("{}", name);
}
