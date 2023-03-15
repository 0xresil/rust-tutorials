### There are some RUST example code here.

#### Run like this
```sh
cargo run --example enums
cargo run --example iterator
cargo run --example ex_sqlx
...

```

#### You can learn about RUST coding from here.

# Rust programming language

## Installation
### Linux or MacOS
* Install
```console
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

* Update using `$ rustup update`

## Commands
* View installed version via `$ rustup show`
* Check latest version via `$ rustup check`

>  Often, `cargo check` is much faster than `cargo build`, because it skips the step of producing an executable. If you’re continually checking your work while writing the code, using `cargo check` will speed up the process! As such, many Rustaceans run `cargo check` periodically as they write their program to make sure it compiles. Then they run `cargo build` when they’re ready to use the executable.

## Getting Started
### Code
```rs
fn main() {
	println!("Hello World!");
}
```

### Compile
```console
$ rustc hello.rs
```

### Output
```console
$ ./hello
```

## Concepts

* `..` used for range like `1..4` i.e. 1, 2, 3. But, if `1..=4` i.e. 1, 2, 3, 4
* There are different types of struct
  * normal struct: with parameters
  * unit struct: without parameters

> “Ownership is Rust’s most unique feature, and it enables Rust to make memory safety guarantees without needing a garbage collector.”

> <u>Borrow Checker</u>: You can move the data itself and give up ownership in the process, create a copy of the data and pass that along, or pass a reference to the data and retain ownership, letting the recipient borrow it for a while. The most appropriate approach depends entirely on the situation. Try [this](./tuts/functions/borrow_checker.rs)

> - Stack (fixed size like char, bool, int; less costly; quick to access by calling var like easy to copy the var) | Heap (variable size like string, list, class; more costly; access var or object via pointer)

* By default, all the variables are defined as `immutable` equivalent to `const` in JS/TS.
* In Rust, borrowing is analogous to referencing in C++ & dereferencing is same as that of C++.
* The value of mutable variable can be changed, but not the type.
* In Rust, every value has a single owner that determines its lifetime.
* The memory of the declared variables are dropped (or freed) when the program leaves a block in which the variable is declared. 
    - E.g. Normally, inside the `main` function, whenever a variable is defined, it is dropped after exiting the `main` function.
```rs
fn main() {
    // Case-1
    let x = 10;
    let r = &x;

    let k;
    {
        let y = Box::new(5);            // Using Box pointer for storing into heap
        let y = 5;              // stored in stack
        // let y <'a> = 5;
        // k = &y;         // y dropped here as it is not available for lifetime. Moreover the block is getting over after this
        k = y;          // this implies that the ownership of 5 is transferred to `k` from `y`
    }
}
```
* Rust doesn't allow _dangling pointer_ by design. This means that any variable, struct, enum, etc can't live more than the lifetime of the referenced type
```rs
struct Config {

}

// INCORRECT ❌
struct App {
    config: &Config     // `Config` used as reference
}

// CORRECT ✅
/// Here, it is used as lifetime ownership of the code.
struct App<'a> {
    config: &'a Config
}
```
* `lifetimes` are a compile-time feature and don’t exist at runtime.
* Rust memory safety is based on this rule: Given an object T, it is only possible to have one of the following:
  - Having several immutable references (&T) to the object (also known as aliasing).
  - Having one mutable reference (&mut T) to the object (also known as mutability).
* Apply `#[derive(Debug)]` for making the struct, enum printable
* Apply `#[derive(Clone)]` for making the struct, enum copyable. 
* `Option` vs `Result`

| Option | Result |
|--|--|
| Some or None | Ok or Err |
| An optional value can have either Some value or no value/ None. | A result can represent either success/ Ok or failure/ Err 
| The Option type is a way to use Rust’s type system to express the possibility of absence | Result expresses the possibility of error |
| mainly used for var, function output. For struct, the parameters can have Option type. E.g. In full name, middle_name can be missing for cases, so define `middle_name: Option<String>`| mainly used for operation, function. As normally a variable won't have Err unless there is some calculation involved with this var |
| Don't want to print the exact issue as `None` doesn't have anything as param unlike `Some(T)` | Want to print the exact issue as `Err(E)` contains the message inside |
| E.g. "./tuts/error_handling/opt" | E.g. "./tuts/error_handling/res" |

### Basics
#### Primitive types and Variables
1. Various sizes of integers, signed and unsigned (i32, u8, etc.)
1. Floating point types f32 and f64.
1. Booleans (bool)
1. Characters (char). Note these can represent unicode scalar values (i.e. beyond ASCII)

#### Print
* 1. formatting variables inside `println` function
```
let name = "Abhijit";
let age = 28;

println!("My name is {name}, and age is {age}");					// ❌
println!("My name is {0}, and age is {1}", name, age);		// ✔️
println!("My name is {}, and age is {}", name, age);			// ✔️
```
* 2. Multiple usage of variables without repetition
```
let alice = "Alice";
let bob = "Bob";

println!("{0}, this is {1}. {1}, this is {0}", alice, bob);
```


#### Variables


## Tools
* Check behind-the-code for a code snippet - https://play.rust-lang.org/
  - Tools >> Expand Macros

## Troubleshoot
### 1. warning: path statement with no effect
* _Cause_: there is a statement having no effect
* _Solution_: Assign the variable to `_`.

Before:
```rs
    let result = match grade {
        "A" => { println!("Excellent!"); },
        "B" => { println!("Great!"); },
        "C" => { println!("Good"); },
        "D" => { println!("You passed"); },
        "F" => { println!("Sorry, you failed"); },
        _ => { println!("Unknown Grade"); }
    };

    result;
```

After:
```rs
    let result = match grade {
        "A" => { println!("Excellent!"); },
        "B" => { println!("Great!"); },
        "C" => { println!("Good"); },
        "D" => { println!("You passed"); },
        "F" => { println!("Sorry, you failed"); },
        _ => { println!("Unknown Grade"); }
    };

    // result;             // warning: path statement with no effect, Solution --> assign to `_` 
    let _ = result;

```

### 2. warning: variant is never constructed, error[E0277]: `UsState` doesn't implement `Debug`
* _Cause_: It simply means that the variant is never used, "constructed", anywhere in your program. There is no `AppAction::Task` anywhere in the program. Rust expects that if you say an enum variant exists, you will use it for something somewhere.
* _Solution_: by putting this before the enum, or individually before intentionally unused items, you can make the warning disappear:

Before:
```rs
enum UsState {
	California,
	Mexico,
	Alaska,
}

enum Coin {
	Penny,
	Nickel,
	Dime,
	Quarter,
	Custom(UsState),
}
```

After:
```rs
#[allow(dead_code)]
#[derive(Debug)]		// this use is recommended, otherwise there is error.
enum UsState {
	California,
	Mexico,
	Alaska,
}

#[allow(dead_code)]
enum Coin {
	Penny,
	Nickel,
	Dime,
	Quarter,
	Custom(UsState),
}
```

### 3. Error: "move occurs...which does not implement the Copy trait"
* _Cause_: Copy designates types for which making a bitwise copy creates a valid instance without invalidating the original instance.

This isn't true for String, because String contains a pointer to the string data on the heap and assumes it has unique ownership of that data. When you drop a String, it deallocates the data on the heap. If you had made a bitwise copy of a String, then both instances would try to deallocate the same memory block, which is undefined behaviour.

* _Solution_: Just use `format` like this:

Before:
```rs
impl Detail for Car {
    fn brand(&self) -> String {
        return self.brand;           
    }
    fn color(&self) -> String {
        return self.color;
    }    
}
```

After:
```rs
impl Detail for Car {
    fn brand(&self) -> String {
        // using `format` instead of directly returning the brand bcoz it throws error:
        // "move occurs because `self.brand` has type `String`, which does not implement the `Copy` trait"
        return format!("{}", self.brand);           
    }
    fn color(&self) -> String {
        return format!("{}", self.color);
    }    
}
```

## References
* [Rust by example](https://doc.rust-lang.org/stable/rust-by-example/)
* [Book: The Rust Programming Language](https://doc.rust-lang.org/book/)
* [Rust for Haskell Programmers!](https://mmhaskell.com/rust)
	- [Part 1: Basic Syntax](https://www.mmhaskell.com/rust/syntax)
	- [Part 2: Managing Memory](https://www.mmhaskell.com/rust/memory)
	- [Part 3: Data Types](https://www.mmhaskell.com/rust/data)
	- [Part 4: Cargo Package Manager](https://www.mmhaskell.com/rust/cargo)
	- [Part 5: Collections and Lifetimes](https://www.mmhaskell.com/rust/lifetimes)
* [Learn Rust Documentation](https://learning-rust.github.io/)
* [Rustlings](https://github.com/rust-lang/rustlings)
* [24 days of Rust](https://zsiciarz.github.io/24daysofrust/index.html)
* [Learn Rust by aml3](https://aml3.github.io/RustTutorial/html/toc.html)
* [Rust for C++ programmers](https://github.com/nrc/r4cppp)
* [Learn Rust by KODERHQ](https://www.koderhq.com/tutorial/rust/)
* [Learn Rust by Practical Projects](https://www.youtube.com/watch?v=LPzx2Fzd7Vs&list=PLK_g1a_cAfaZDdybJzwI1m7AVl4tSo87Z)
* [Learn Rustlings](https://www.youtube.com/watch?v=VZnfLBmc_Oo&list=PLSbgTZYkscaoV8me47mKqSM6BBSZ73El6&index=12)
* [Learn Rust by Book via Video](https://www.youtube.com/watch?v=5QsEuoIt7JQ&list=PLSbgTZYkscaoV8me47mKqSM6BBSZ73El6&index=1)

### Blogs
* [What is Rust and why is it so popular?](https://stackoverflow.blog/2020/01/20/what-is-rust-and-why-is-it-so-popular/)
* [Understanding the Rust borrow checker](https://blog.logrocket.com/introducing-the-rust-borrow-checker/)
* [No auto type deduction for function, but for local variable](https://stackoverflow.com/questions/24977365/differences-in-type-inference-for-closures-and-functions-in-rust)
* [Including Files and Deeply Directories in Rust](https://hackernoon.com/including-files-and-deeply-directories-in-rust-q35o3yer)
* [Understand Rust Ownership model by thoughtram](https://blog.thoughtram.io/rust/2015/05/11/rusts-ownership-model-for-javascript-developers.html)
* [Memory Safety in Rust: A Case Study with C](https://willcrichton.net/notes/rust-memory-safety/)
* [Ownership in Rust by thoughtram](https://blog.thoughtram.io/ownership-in-rust/)
* [References in Rust by thoughtram](https://blog.thoughtram.io/references-in-rust/)
* [Iterators in Rust by thoughtram](https://blog.thoughtram.io/iterators-in-rust/)
* [Lifetimes in Rust by thoughram](https://blog.thoughtram.io/lifetimes-in-rust/)
