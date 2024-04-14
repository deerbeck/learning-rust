

fn cartesian() -> (i32, i32) {
    return (1,5);
}

fn main() {
    let (_x, y) = cartesian();

    if y > 5 {
        println!("y is greater than 5");
    } else if y < 5 {
        println!("y is less than 5");
    } else {
        println!("y is 5");
    }
}
