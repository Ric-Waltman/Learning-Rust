# Learning-Rust
Projects and exercises from, *The Rust Programming Language*

## Chapters
### Chapter 1: Getting Started

**rustup**: Rust's toolchain manager
 - Manages rust toolchain versions, like the compiler, package manager, and formatter

**rustc**: Rust's compiler

**cargo**: Rust's package manager (w/ compiler built-in)
 - *cargo build* : builds the source code
	 - *--release* : this flag optimizes code without debug information
 - *cargo run* : builds (if needed) the source code, and runs it in 1 step
 - *cargo check* : checks that code compiles, but doesn't spend time building

### Chapter 3: Common Programming Concepts
**3.1: Variables and Mutability**
Variables are immutable by default. `let x = 5;` is an immutable declaration, and `x` can not be reassigned or altered. Rust compiler guarantees this immutability. Code is easier to reason about due to this. Keyword `mut` is used to make a mutable variable: `let mut x = 5;`

Constants are always immutable. No `mut` keyword allowed. `const MAX_POINTS: u32 = 5;` is how you declare a const variable. The data type must be annotated for consts. Consts can not be set to values determined at runtime, like function calls. They can only be set to a constant expression.

**Shadowing** allows us to perform transformations on an immutable variable, without losing the immutability of the variable. The value is used as a copy before being reassigned, after the calculation, to the original variable. We're effectively creating a new variable each time we shadow, and all we do is re-use the name. A shadowed variable can be of a different type, as well.

**3.2: Data Types**
Rust is *statically typed*, meaning all variable types must be known at compile time. Sometimes type annotations are needed for the compiler to determine the right type.

Rust has *scalar types* and *compound types*.

**Scalar Types** include: integers, floating-point numbers, Booleans, and characters. `isize` and `usize` are architecture-specific: 64-bit or 32-bit. Although, 32-bit is generally the fastest, even on 64-bit systems. isize and usize are good when indexing some sort of collection.

**Integer Literals** can be written in Decimal, Hex, Octal, Binary, and Byte. All literals except byte literals allow type suffixes, like `57u8`. And `_` can be a visual separator for large numbers, like 1,000: `1_000`

**Integer Overflows** cause a *panic* in debug mode at runtime if the behavior occurs. **Release** does not include these panics.

**Floating-Point Types** - Rust defaults to `f64` because on modern CPUs it's the same speed as `f32` but with more precision. 

**Booleans** are 1 byte, with possible values: `true` & `false`

**Character Type**: literals are specified with single quotes: `let c = 'z';`, not double quotes like string literals. 4 bytes long, and is a Unicode Scalar Value, meaning it can represent more than ASCII. It can support accented letters and emojis! 

**Compound Types**: *Tuple* groups together multiple values of a variety of types into one compound type. They have a fixed length, and can not grow or shrink in size.
*Arrays* are for the same data type, and also have static size. They are stack-allocated. Indexing beyond the bounds of an array results in a *runtime error*, panicking out of the program.

**3.3: Functions**
Rust uses *snake_case_function_name()* as the conventional style for function and variable names. Rust doesn't care about where functions are defined and used, like C does (requiring prototypes). So long as the function is defined *somewhere*, Rust will find it.

Functions signatures must have the type of each parameter. This means that you almost never need types elsewhere in the code, as the compiler can use the function signature information to inform itself.

*Statements* are instructions that perform some action and do no return a value. Ex: `let y = 6`. Writing `let x = (let y = 6);` results in an error. This is different than C, where assignment returns the value of assignment, i.e. `x = y = z = 0;`
*Expressions* evaluate to a resulting value. Ex: 

*Return Type* can be either tha last implicit expression within the function, or the keyword `return` to return early

**3.4: Comments** are the same as C, `\\`

**3.5: Control Flow**
Rust does not try to automatically convert non-Boolean types to Booleans. `if` is an expression, not a statement.
*Looping* options: `loop` executes block of code forever until you explicitly tell it to stop. `while` and `for` are largely the same as C.

### Chapter 2: Programming a Guessing Game
Rust *Crates* are collections of Rust source code files. This project is a *binary crate*, which is an executable. The `rand` crate is a *library crate*.

*Cargo.lock* ensures we can rebuild the same artifact every time anyone builds this code. If rand goes to version 0.5.6 tomorrow, and has a regression that break our code, *Cargo.lock* will be written with the exact versions used during the first project build. If you *do* want to update, `cargo update` will ignore Cargo.lock and figure out the latest versions for all dependencies in *Cargo.toml*

You won't know which traits to use right off the back, but `cargo doc --open` will build documentation provided by all dependencies locally and open them in the browser.

