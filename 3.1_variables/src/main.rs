fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);


    let y = 15;
    let y = y + 1; // shadow's the first y declaration
    let y = y * 2; // shadow's the 

    println!("The value of y is: {}", y);
}
