use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    // Gives us a thread-local rng seeded by the OS, and generated a rand num between 1 & 100
    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Guess the number!");

        let mut guess = String::new(); // new is implemented on the type, not a particular instance of String. AKA a 'static method'
        
        // - &mut guess is a 'reference' to mutable, so no copies here
        // - read_line() returns an io::Result, an enum type. If io::Result is `Err` , as opposed to `Ok`, the program will crash
        //   and print out the string literal passed as an argument. If io::Result is `Ok`, expect() will return the value of the
        //   successful read_line() call to be used.
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
        
        // - We get to shadow original 'guess' and re-use the name as type u32 instead of string
        // - trim() eliminates any whitespace, including '\n', at the beginning and end of input
        // - parse() parses a string into some kind of number. We must tell Rust which type, done by declaring 'guess' as 'u32'
        // - match allows us to handle the Result type (error handling!!!)
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, // '_' catches all Err values, no matter the information inside
        };

        // Ordering is an enum, with variants Less, Greater, and Equal
        //
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!\n"),
            Ordering::Greater => println!("Too big!\n"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
