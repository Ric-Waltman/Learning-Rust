fn main() {
    println!("Hello, world!");
    if_examples();
    loop_example();
    while_example();
    for_example();
}

fn if_examples() {
    let n = 3;

    if n < 5 {
        println!("n is < 5");
    } else {
        println!("n is >= 5");
    }

    // Illustrating how 'if' is an expression
    let b = true;
    // returns values from if branches must be of the same type
    let x = if b {
        1
    } else {
        0
    };
    println!("x: {}", x);
}

fn loop_example() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 5;
        }
    };

    println!("result: {}", result);
}

fn while_example() {
    let mut counter = 0;

    while counter != 10 {
        println!("{}!", counter);
        counter += 1;
    }

    println!("counter: {}", counter);
}

fn for_example() {
    let a = [10, 20, 30, 40, 50];

    // For loop using array's iterator
    for elem in a.iter() {
        println!("{}", elem);
    }

    // For loop using a Range (std type), reversing the range
    for i in (0..5).rev() {
        println!("{}", i);
    }
}
