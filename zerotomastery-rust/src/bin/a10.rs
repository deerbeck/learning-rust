fn print_words(x: bool) 
{
    match  x {
        true => println!("its big"),
        false => println!("its small"),
    }
}

fn main() {
    let number: i32 = 101;
    let num_size: bool = number > 100;
    print_words(num_size);
}
