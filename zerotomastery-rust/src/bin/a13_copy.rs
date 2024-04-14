// Topic: Vectors
//
// Requirements:
// * Print 10, 20, "thirty", and 40 in a loop
// * Print the total number of elements in a vector
//
// Notes:
// * Use a vector to store 4 numbers
// * Iterate through the vector using a for..in loop
// * Determine whether to print the number or print "thirty" inside the loop
// * Use the .len() function to print the number of elements in a vector

struct Temperature {
    degrees_f: f64,
}

impl Temperature {
    fn freezing() -> Self {
        Self { degrees_f: 32.0 }
    }
    fn boiling() -> Self {
        Self { degrees_f: 212.0 }
    }
    fn show_temp(&self) {
        println!("{:?} degrees F", self.degrees_f)
    }
}

fn main() {
    let hot: Temperature = Temperature { degrees_f: 99.9 };
    hot.show_temp();
    hot.show_temp();
    
    let cold: Temperature = Temperature::freezing();
    cold.show_temp();
    
   let boiling: Temperature = Temperature::boiling();
   boiling.show_temp();
}
