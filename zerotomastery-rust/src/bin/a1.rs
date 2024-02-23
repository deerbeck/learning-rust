// Topic: Functions
//
// Program requirements:
// * Displays your first and last name
//
// Notes:
// * Use a function to display your first name
// * Use a function to display your last name
// * Use the println macro to display messages to the terminal

fn first_name() {
    println!("Johannes")
}
fn last_name() {
    println!("Hirschbeck")
}
fn sub(a: i32, b: i32) -> i32 {
    a - b
}

fn main() {
    //first_name();
    //last_name();

    let sum = 2 + 2;
    let value = 10 - 5;
    let division = 10 / 5;
    let multiplication = 5 * 5;

    let five = sub(8, 3);

    let rem = 6%3;
}
