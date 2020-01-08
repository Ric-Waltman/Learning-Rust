fn main() {
    let coin = Coin::Quarter(State::Alaska);
    println!("value of coin: {}", value_in_cents(&coin));

    let five = Some(5);
    let six = plus_one(five);

    // Shorthand for matching and binding a single variant
    if let Coin::Quarter(state) = coin {
        println!("if let found: {:?}", state);
    }
}

// ---------------------------------------------------------------------------
#[derive(Debug)] // to inspect state while debugging
enum State {
    Alabama,
    Alaska,
    // ...
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(State),
}

// ---------------------------------------------------------------------------
fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,

        // This branch binds `state` variable to the value of the enum variant
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

// ---------------------------------------------------------------------------
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        None => None
    }
}


