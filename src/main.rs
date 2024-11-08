use std::io;

fn main() {
    let mut counter = 0;

    loop {
        println!("\nCurrent counter value: {}", counter);
        println!("Choose an option:");
        println!("1. Increase counter");
        println!("2. Decrease counter");
        println!("3. Reset counter");
        println!("4. Exit");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read input");
        let choice = choice.trim();

        match choice {
            "1" => {
                counter += 1;
                println!("Counter increased!");
            }
            "2" => {
                counter -= 1;
                println!("Counter decreased!");
            }
            "3" => {
                counter = 0;
                println!("Counter reset!");
            }
            "4" => {
                println!("Exiting...");
                break;
            }
            _ => println!("Invalid choice, please try again."),
        }
    }
}
