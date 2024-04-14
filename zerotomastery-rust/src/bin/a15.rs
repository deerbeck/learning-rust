enum Ticket {
    Backstage(String, i32),
    VIP(String, i32),
    Standard(i32),
}

fn main() {
    let audience: Vec<Ticket> = vec![
        Ticket::Backstage(String::from("Emma"), 30),
        Ticket::VIP(String::from("John"), 50),
        Ticket::Standard(10),
    ];

    for person in audience {
        match person {
            Ticket::Backstage(name, price) => println!(
                "Backstage Ticket Holder: {:?} Backstage Ticket Price: {:?}",
                name, price
            ),
            Ticket::VIP(name, price) => {
                println!(
                    "VIP Ticket Holder: {:?} VIP Ticket Price: {:?}",
                    name, price
                )
            }
            Ticket::Standard(price) => println!("Standard Ticket Price: {:?}", price),
        }
    }
}
