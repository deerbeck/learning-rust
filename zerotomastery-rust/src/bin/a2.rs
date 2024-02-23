// Topic: Basic arithmetic
//
// Program requirements:
// * Displays the result of the sum of two numbers
//
// Notes:
// * Use a function to add two numbers together
// * Use a function to display the result
// * Use the "{:?}" token in the println macro to display the result
fn sum(a: i32, b: i32) -> i32 {
    a + b
}

fn print_res(res: i32) {
    println!("{:?}", res);
}

fn main() {
    //print_res(sum(1, 2));
    let age = 15;
    if age >= 21 {
        println!("ok to purchase");
    } else {
        println!("cannot purchase");
    }
}
