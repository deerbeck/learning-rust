// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor
enum Flavors {
    Banana,
    Chocolate,
    Orange,
}
struct Drink {
    flavor: Flavors,
    ounces: f32,
}

fn drink_information(drink: Drink) {
    match drink.flavor {
        Flavors::Banana => println!("flavor: banana"),
        Flavors::Chocolate => println!("flavor: chocolate"),
        Flavors::Orange => println!("flavor: orange"),
    }
    println!("ounces: {:?}", drink.ounces);
}

fn main() {
    let drink = Drink {
        flavor: Flavors::Banana,
        ounces: 1.3,
    };
    drink_information(drink);
}
