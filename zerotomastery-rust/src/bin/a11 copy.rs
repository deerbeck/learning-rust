// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter

struct Favorites {
    color: String,
    number: i32,
    state: String,
    food: String,
    tv_show: String,
    place: String,
}

fn main() {
    let coord: (i32, i32) = (2, 3);
    println!("{:?}, {:?}", coord.0, coord.1);

    let (x, y) = (2, 3);
    println!("{:?}, {:?}", x, y);

    let user_info = ("Emma", 20);
    println!("Name: {:?}\nAge: {:?}", user_info.0, user_info.1);

    let (name, age) = ("Emma", 20);
    println!("Name: {:?}\nAge: {:?}", name, age);

    let favorite: (&str, i32, &str, &str, &str, &str) =
        ("red", 14, "TX", "pizza", "TV Show", "home");

    let state: &str = favorite.2;
    let place: &str = favorite.5;
    let emma: Favorites = Favorites {
        color: String::from("red"),
        number: 14,
        state: String::from("TX"),
        food: String::from("pizza"),
        tv_show: String::from("TV show"),
        place: String::from("home"),
    };

    println!("{}, {}, {}", emma.color, emma.state, emma.number);
}
