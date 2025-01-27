
#### Error Handling

Rust groups errors into two major categories:
 * `recoverable` errors such as a `file not found error`, just want to report the problem to the user and retry the operation
 * `unrecoverable` errors are always symptoms of bugs, such as trying to access a location beyond the end of an array, and so we want to immediately stop the program

 Rust doesn't have exceptions. Instead, it has the type `Result<T, E>` for recoverable erros and the `panic!` macro that stops execution when the program encounters an unrecoverable error.

##### Unrecoverable Errors with panic!

There are two ways to cause a panic in practice:
 * by taking an action that causes our code to panic
 * by explicitly calling the `panic!` macro

By default, these panics will print a failure message, unwind, clean up the stack, and quit.
Via environment variables, you can also have Rust display the call stack when a panic occurs to make it easier to track down the source of the panic.

> **Unwinding the Stack or Aborting in Response to a Panic**   
> By default, when a panic occurs the program starts `unwinding`. However, walking back and cleaning up is a lot of work. Rust, therefore, allows you to choose the alternative of immediately `aborting`, which ends the program without cleaning up.   
>   
> you can switch from unwinding to aborting upon a panic by adding `panic = 'abort'` to the appropriate `[profile]` sections in your `Cargo.toml` file.   

##### Recoverable Errors with Result

Sometimes when a function fails it's for a reason that you can easily interpret and respond to.   
For example, if you try to open a file and that operation fails because the file doesn't exist, you might want to create the file instead of terminating the process.   

```rust
// Result enum
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

The `T` and `E` are generic type parameters. `T` represents the type of the value that will be returned in a success case within the `Ok` variant, and `E` represents the type of the error that will be returned in a failure case within the `Err` variant. 

```rust
use std::fs::File;

fn main() {
    let greeting_file_result = File::open("hello.txt");
}
```

`T`: `std::fs::File` which is a file handle.   
`E`: `std::io::Error` which has error information.   

##### Shortcuts for Panic on Error: unwrap and expect

Using `match` works well enough, but it can be a bit verbose and doesn't always communicate intent well.
The `Result<T, E>` type has many helper methods defined on it to do various, more specific tasks.
 * The `unwrap` method is a shortcut method implemented just like the `match` expression. 
    - If the `Result` value is the `Ok` variant, `unwrap` will return the value inside the `Ok`.
    - If the `Result` is the `Err` variant, `unwrap` will call the `panic!` macro for us.
 * The `expect` method lets us also choose the `panic!` error message with `unwrap`


##### Propagating Errors

When a function's implementation calls something that might fail, instead of handling the error within the function itself you can return the error to the calling code so that it can decide what to do.   
This is known as `propagating` the error and gives more control to the calling code, where there might be more information or logic that dictates how the error should be handled than what you have available in the context of your code. 

 * return `Result<T, E>` in function
 * using shortcut `? operator`

##### Where The ? Operator Can Be Used

The `?` operator can only be used in functions whose return type is compatible withe the value the `?` is used on. This is because the `?` operator is defined to perform an early return of a value out of the function, in the same manner as the `match` expression.

Rust only allows to use the `?` operator in a function that returns `Result`, `Option`, or another type that implements `FromResidual`.   


##### Guidelines for Error Handling

A `bad state` is when some assumption, guarantee, contract, or invariant has been broken, such as when invalid values, contradictory values, or missing values are passed to your code - plus one more of the following:
 * The bad state is somethting that is unexpected, as opposed to something that will likely happen occasionally, like a user entering data in the wrong format.
 * Your code after this point needs to rely on not being in this bad state, rather than checking for the problem at every step.
 * There's not a good way to encode this information in the types you use.

In cases where continuing could be insecure or harmful, the best choice might be to call `panic!` and alert the person using your library to the bug in their code so they can fix it during development. Similarly, `panic!` is often appropriate if you're calling external code that is out of your control and it returns an invalid state that you have no way of fixing.

However, when failure is expected, it's more appropriate to return a `Result` than to make a `panic!` call. Examples include a parser being given malformed data or an HTTP request returning a status that indicates you have hit a rate limit. In these cases, returning a `Result` indicates that failure is an expected possibility that the calling code must decide how to handle.


##### Creating Custom Types for Validation

before
```rust
loop {
    // --snip--

    let guess: i32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };

    if guess < 1 || guess > 100 {
        println!("The secret number will be between 1 and 100.");
        continue;
    }

    match guess.cmp(&secret_number) {
        // --snip--
    }
}

```

after
```rust
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {value}.");
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}
```
