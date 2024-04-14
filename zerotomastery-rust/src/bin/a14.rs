// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

struct Person {
    name: String,
    age: i32,
    fav_color: String,
}

fn print(data: &str) {
    println!("{:?}", data);
}

fn main() {
    let people = vec![
        Person {
            name: String::from("John"), 
            age: 20,
            fav_color: String::from("Blue"),
        },
        Person {
            name: String::from("Sean"),
            age: 25,
            fav_color: String::from("Green"),
        },
        Person {
            name: String::from("Anna"),
            age: 27,
            fav_color: String::from("Yellow"),
        },
    ];

    for person in people {
        if person.age >= 22 {
            print(&person.name);
            print(&person.fav_color);
        }
    }
}
