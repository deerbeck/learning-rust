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

enum Access {
    Admin,
    Manager,
    User,
    Guest,
}

fn main() {
    // secret file: admins only
    let access_level: Access = Access::Guest;
    let can_access_file: bool = match access_level {
        Access::Admin => true,
        _ => false,
    };

    println!("{}", can_access_file)
}
