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

struct Grocery {
    quantity: i32,
    id: i32,
}

fn display_quanity(item: &Grocery)
{
    println!("{}", item.quantity);
}


fn display_id(item: &Grocery)
{
    println!("{}", item.id);
}

fn main() {
    let item: Grocery = Grocery
    {
        quantity: 12,
        id: 4123,
    };

    display_quanity(&item);
    display_id(&item);
}
