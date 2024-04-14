// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

struct Student {
    name: String,
    locker: Option<i32>,
}

fn main() {
    let students = vec![
        Student {
            name: "Anna".to_owned(),
            locker: None,
        },
        Student {
            name: "John".to_owned(),
            locker: Some(24),
        },
    ];

    for stud in students {
        println!("Student Name: {:?}", stud.name);
        match stud.locker {
            Some(num) => println!("Locker number: {:?}", num),
            None => println!("No Locker Assigned!"),
        };
    }
}
