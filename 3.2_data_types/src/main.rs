fn main() {
    let x = 3.019283754908127365908; // f64
    let y: f32 = 3.019283754908127365908; // f32

    println!("double x: {}", x); // prints up to 3.0192837549081273
    println!("float  y: {}", y); // prints up to 3.0192838

    let f: bool = false;
    println!("f: {}", f);

    // Tuples can hold different data types, but size is static
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // Destructure the tuple
    let (a,b,c) = tup;

    // Print the tuple using destructures variables, and also .index
    println!("tup: {:?}, tup.0: {}, tup.1: {}, tup.2: {}", tup, tup.0, tup.1, tup.2);
    println!("a: {}, b: {}, c: {}", a, b, c);

    // Arrays hold the same data type, and size is static (like C arrays)
    // - lives on the stack
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let zero_arr: [u32; 10] = [0; 10]; // initializes an array, all 0s, with length of 10

    // Access arrays by indexing the same as C++
    println!("arr[0]: {}", arr[0]);
    println!("zero_arr[9]: {}", zero_arr[9]);
}


