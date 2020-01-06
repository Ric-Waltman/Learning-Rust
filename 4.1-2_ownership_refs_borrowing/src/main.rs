fn main() {
    // Two variables with the same value, both on the stack
    // Compile time (stack) values are cheap and quick to make. Calling clone() would not do anything differently.
    let _x = 5;
    let _y = _x; // Copies x's value and binds that value to y

    // This ONLY copies the String stack data: {ptr, len, capacity}, not the string which resides on the stack
    let s1 = String::from("hello");

    // After this s1 move into s2, s1 is invalid and can not be used.
    let s2 = s1;

    println!("{}, world!", s2);

    // Clone method performs a deep copy, including the heap data
    let p1 = String::from("ricky");
    let p2 = p1.clone();
    println!("p1: {}, p2: {}", p1, p2);

    takes_ownership(s2); // s2's value moved into the function, and is no longer valid after the call

    makes_copy(_x); // _x would move into the function, but i32 is Copy trait, so no move occurs. _x still valid after call

    let p1 = takes_and_gives_back(p1); // p1 is moved into the function
    println!("p1 after takes_and_gives_back(): {}", p1);

    let s3 = gives_ownership(); // Ownership from a variable declared within this function is returned
    println!("s3: {}", s3);

    // References take no ownership, so p1 is never moved. It is 'borrowed'
    let len = reference_takes_no_ownership(&p1);
    println!("len: {}", len);
    println!("p1 after reference_takes_no_ownership(): {}", p1);
}

fn takes_ownership(in_str: String) { // in_str comes into scope
    println!("takes_ownership(): {}", in_str);
} // in_str goes out of scope and `drop` is called. The backing memory is freed, and can not be used after this function

fn makes_copy(int: i32) { // int comes into scope
    println!("makes_copy(): {}", int);
} // int goes out of scope, is popped off of stack after function exits

fn takes_and_gives_back(in_str: String) -> String {
    in_str + " waltman" // in_str is returned and moves out to the calling function
}

fn gives_ownership() -> String {
    String::from("str from gives_ownership() scope")
}

fn reference_takes_no_ownership(in_str: &String) -> usize { // in_str is a reference to a String
    in_str.len()
} // in_str goes out of scope. But nothing happens because it has no ownership of what it refers to.