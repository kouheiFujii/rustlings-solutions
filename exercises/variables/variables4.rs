// variables4.rs
// Execute `rustlings hint variables4` or use the `hint` watch subcommand for a hint.

fn main() {
    // by default, variables are immutable
    // you can make them mutable by adding `mut` before the variable name
    // let x = 3;
    let mut x = 3;
    println!("Number {}", x);
    x = 5; // don't change this line
    println!("Number {}", x);
}
