fn main() {
    println!("Hello, world!");

    another_function();
    function_with_parameter(87, true);
    statements_and_expressions();
    
    let res = fn_returns_bool();
    println!("fn_returns_bool(): {}", res);
}

fn another_function() {
    println!("another_function() called!");
}

fn function_with_parameter(x: i32, b: bool) {
    println!("function_with_parameter(x: i32) received x = {}, b = {}", x, b);
}

fn statements_and_expressions() {
    let x = 5;

    let y = { // new scope
        let x = 13; // standard statement
        x + 1 // notice there's no semicolon. Semicolons are for statements only, not expressions
    };

    println!("x: {}", x);
    println!("y: {}", y);
}

fn fn_returns_bool() -> bool {
    true // implicit expression returns from function. `return` keyword can also be used
}