fn main() {
    let s = String::from("hello world");

    // Create String 'Slices', references to just part of the string
    let _hello = &s[..5];  // no starting_range is needed if coming from 0
    let _world = &s[6..11];
    let _world2 = &s[6..]; // no ending_range is needed if going to max index of the collection
    let _whole_s = &s[..]; // both are dropped, resulting in a slice of the whole string

    println!("first word: '{}'", first_word(&s[..]));

    let string_literal = "string literal";

    // function accepts slices of string literals (aka slices)
    println!("first word: '{}'", first_word(&string_literal[..]));
    println!("first word: '{}'", first_word(string_literal));

    // Array slice
    let a = [1, 2, 3, 4, 5];
    let a_slice = &a[2..];
    println!("a_slice: {:?}", a_slice);
}

// &str means string slice
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
