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

### Chapter 4: Understanding Ownership
*Ownership* is Rust's standout feature, and enables memory safety guarantees without a garbage collector. The system of ownership itself has a set of rules enforceable by the compiler. These features **do not** slow down your program while it's running. 

**Stack** is *LIFO*, *last in, first out*, just like a stack of plates. Data with unknown or changing size at runtime must be stored on the **heap**. The heap is less organized, and the OS just finds an empty spot of the necessary size to use when needed. This is known as *allocating*. Pushing values onto the stack is **NOT** considered allocating. Pushing to the stack is faster than allocating on the heap because the OS doesn't have to search for a place to store new data. Accessing data in the *heap* is also slower than on the stack, because you have to follow a pointer for the heap data. Processors are faster when they don't jump around as much. Recall CPU architecture and locality. Ultimately, **managing heap data is why ownership exists and helps explain why it works the way it does.**

**Ownership Rules**
- Each value in Rust has a variable that's called its *owner*
- There can only be one owner at a time
- When the owner goes out of scope, the value will be dropped

Note: The reason for memory issues comes down to asking the OS to do things for us on a certain memory region. We need to pair exactly one `allocate` with exactly one `free`. No double frees, memory leaks, or invalid variables.

In Rust, the memory is automatically returned once the variable that owns it goes out of scope. In Rust, this memory de-allocation is called `drop`. This is the same as C++'s *Resource Acquisition Is Initialization (RAII)*, which deallocates resources at the end of an item's lifetime.

Rust never automatically creates *deep* copies of your data. Everything is a move by default, and thus inexpensive in terms of runtime performance. If we **do** want to perform a *deep* copy, including copying the heap data, we can use the `clone` method.

**Copy** trait allows an older variable to be used even after assignment. You can not implement *both* the **Copy** and **Drop** traits.

When a variable that includes data on the heap goes out of scope, the value will be cleaned up by `drop` unless the data has been moved to be owned by another variable.

**References and Borrowing**
Just like C++, Rust has references as arguments to functions. This allows function parameters to use a value without taking ownership of it. The `&` operator creates a reference that refers to the value of a variable, but does not own it. Reference parameters aresaid to **borrow** the given value. We can NOT edit a borrowed value. References are immutable by default. **Mutable References** allow a *borrowed* value to be edited, **but** the big limitation is that you can only have one mutable reference to a piece of data in any given scope. The benefit: NO DATA RACES guaranteed by the compiler.

**Data Race** is similar to a race condition, and happens under the following behavior:
- Two or more pointers access the same data at the same time
- At least one pointer is being used to write to that data
- No mechanism to synchronize the data access

You can only have both a *mutable reference* and an *immutable reference* within the same scope **if** the immutable reference is not used after the mutable reference is made.

**Dangling Pointers** are pointers that referene a location in memory that may have been given to someone else. This can be done by freeing some memory while preserving a pointer to that memory. Rust **disallows** this via the compiler ensuring the underlying data does not go out of scope before the reference to the data does.

**Rules of References**
- At any given time you can have *either* one mutable reference *or* n immutable references
- References must always be valid, and not dangle

**The Slice Type** allows you to reference a contiguous sequence of elements in a collection instead of the *entire* collection. We create slices using a *range* with brackets: `[starting_index..ending_index]`, where *ending_index* is non-inclusive. String literals are actually slices: in `let s = "Hello, world!";` the type of `s` is `&str`, because it's a slice pointing the specific part of the built binary where the string literal is. More accurately, `s` is an immutable reference to a slice.

While String Slices are specific to strings, there are other slice types, like in this example:
`let a = [1,2,3,4,5];`
`let slice = &a[1..3];`
slice, here, has type `&[i32]`.

Slices store a reference to the first element and a length.

### Chapter 5: Using Structs to Structure Related Data
A *structure* is a custom data type that lets you name and package multiple related values into a meaningful group. When instantiating a struct, the values don't have to follow the same ordering as the definition. When instantiating a struct, it must either be completely *immutable*, or completely *mutable* - there is no partial in-between.

*Field Init Shorthand* allows struct fields to be initialized by parameter variables when their names match. This method is less verbose.

*Struct update syntax* updates remaining fields from an existing instantiation of a struct. Uses `..instantiation1` syntax.

**Tuple Structs** are essentially names tuples, but they lack field names: `struct Color(i32, i32, i32);`. Each tuple struct has it's own type, even if the fields are all the same. Field access is the same as *tuples*.

**Unit-Like Structs** have no fields. They are useful if you want to implement a trait on some type without adding any other data fields.

**Methods** are functions specific to the context of a struct (or enum or trait obj). The first parameter is always `self`, aka `this`, representing the instance of the struct the method is called on. Methods can take ownership of `self`, borrow `self` immutably, or borrow `self` mutably, just as they can any other parameter.

Unlike in C++, we don't need a `->` operator. Rust has **automatic referencing and dereferencing**. When you call `object.something()`, Rust automatically adds in `&`, `&mut`, or `*` so `object` matches the signature of the method. Thus, the following are the same:
`p1.distance(&p2);`
`(&p1).distance(&p2);`

**Associated Functions** are functions defined inside an `impl` block which do **not** take `self` as a parameter. These are useful for writing constructors. These functions are called like statics in C++: `struct_name::assoc_function();`. Note the function is namespaced by the struct with `::`.

You can have multiple separate `impl` blocks for the same `struct` type.