use std::io;
fn main() {
    println!("Hello, world!");
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("failed to read line");
    println!("{}", input);

    let my_half = 0.5;
    let your_half = my_half;
    println!("{}", your_half);
    println!("{:?}", your_half);
}
