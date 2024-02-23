// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print
enum Colors {
    Red,
    Green,
    Blue,
    Yellow,
}

fn what_color(color: Colors) {
    match color {
        Colors::Red => println!("red"),
        Colors::Green => println!("green"),
        Colors::Blue => println!("blue"),
        Colors::Yellow => println!("yellow"),
        _ => println!("other"),
    }
}

fn main() {
    what_color(Colors::Red);
}
