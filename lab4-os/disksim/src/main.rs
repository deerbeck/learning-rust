fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <INITIAL_POSITION>", args[0]);
        std::process::exit(1);
    }

    let initial_position: i32 = args[1].parse().expect("Not a number!");

    let mut fcfs_distance: i32 = 0;
    let mut sstf_distance: i32 = 0;
    let mut scan_distance: i32 = 0;
    let mut c_scan_distance: i32 = 0;
    let mut look_distance: i32 = 0;
    let mut c_look_distance: i32 = 0;

    let mut number_of_cylinders: usize = 100;

    let mut cylinders = vec![0; number_of_cylinders];
    println!("{:?}", cylinders);
    let x = 10;
}
