
### Object-Oriented Programming Features of Rust

Object-oriented programming(OOP) is a way of modeling programs. Objects as a programmatic concept were introduced in the programming language Simula in the 1960s. Those objects influenced Alan Kay's programming architecture in which objects pass messages to each other. To describe this architecture, he coined the term *object-oriented programming* in 1967. Many competing definitions describe what OOP is, and by some of these definitions Rust is object-oriented, but by others it is not.

In this chapter, we'll explore certain characteristics that are commonly considered object-oriented and how those characteristics translate to idiomatic Rust. We'll then show you how to implement an object-oriented design pattern in Rust and discuss the trade-offs of doing so versus implementing a solution using some of Rust's strengths instead.


#### Characteristics of Object-Oriented Languages

Let's look at what each of those characteristics means and whether Rust supports it.


##### Objects Contain Data and Behavior

The book *Design Patterns: Elements of Reusable Object-Oriented Software* defines OOP this way:

> Object-oriented programs are made up of objects. An *object* packages both data and the procedures that operate on that data. The procedures are typically called *methods* or *operations*.

Using this definition, Rust is object-oriented: structs and enums have data, and impl blocks provide methods on stucts and enums. Even though structs and enums with methods aren't called objects, they provide the same functionality, according to GoF's definition of objects.


##### Encapsulation that Hides Implementation Detail

Another aspect commonly associated with OOP is the idea of *encapsulation*, which means that the implementation details of an object aren't accessible to code using that object. Therefore, the only way to interact with an object is through its public API; code using the object shouldn't be able to reach into the object's internals and change data or behavior directly. This enables the programmer to change and refactor an object's internals without needing to change the code that uses the objects.

We can you the `pub` keyword to decide which modules, types, functions, and methods in our code should be public, and by default everything else is private. 

If encapsulation is a required aspect for a language to be considered object-oriented, then Rust meets that requirement. The option to use `pub` or not for different parts of code enables encapsulation of implementation details.


##### Inheritance as a Type System and as Code Sharing

*Inheritance* is a mechanism whereby an object can inherit elements from another object's definition, thus gaining the parent object's data and behavior without you having to do define them again.

If a language must have inheritance to be an object-oriented language, then Rust is not one. There is no way to define a struct that inherits the parent struct's fields and method implementations without using a macro.

However, if you're used to having inheritance in your programming toolbox, you can use other solutions in Rust, depending on your reason for reaching for inheritance in the first place.

You would choose inheritance for two main reasons. One is for reuse of code: you can implemnt particular behavior for one type, and inheritance enables you to reuse that implementation for a different type. You can do this in a limited way in Rust code using default trait method implementations. 

The other reason to use inheritance relates to the type system: to enable a child type to be used in the same places as the parent type. This is also called *polymorphism*, which means that you can substitute multiple objects for each other at runtime if they share certain characteristics.

Inheritance has recently fallen out of favor as a programming design solution in many programming languages because it's often at rixk of sharing more code than necessary. Subclasses shouldn't always share all characteristics of their parent class but will do so with inheritance. This can make a program's design less flexible. It also introduces the possibility of calling methods on subclasses that don't make sense or that cause errors because the methods don't apply to the subclass. In addition, some languages will only allow single inheritance (meaning a subclass can only inherit from one class), further restricting the flexibility of a program's design.

For these reasons, Rust takes the different approach of using trait objects instead of inheritance.


#### Using Trait Objects That Allow for Values of Different Types


##### Defining a Trait for Common Behavior

We create a trait object by specifying some sort of pointer, such as a `&` reference or a `Box<T>` smart pointer, then the `dyn` keyword, and then specifying the relevant trait. We can use trait objects in place of a generic or concrete type. Wherever we use a trait object, Rust’s type system will ensure at compile time that any value used in that context will implement the trait object’s trait. Consequently, we don’t need to know all the possible types at compile time.

```rust
pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
```

##### Implementing the Trait

src/lib.rs
```rust
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // code to actually draw a button
    }
}

```

src/main.rs
```rust
use gui::{Button, Screen};
use gui::Draw;

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // code to actually draw a select box
    }
}

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();
}
```

This concept—of being concerned only with the messages a value responds to rather than the value’s concrete type—is similar to the concept of *duck typing* in dynamically typed languages: if it walks like a duck and quacks like a duck, then it must be a duck! In the implementation of `run` on `Screen`, `run` doesn’t need to know what the concrete type of each component is. It doesn’t check whether a component is an instance of a `Button` or a `SelectBox`, it just calls the draw method on the component. By specifying `Box<dyn Draw>` as the type of the values in the `components` vector, we’ve defined `Screen` to need values that we can call the `draw` method on.


##### Trait Objects Perform Dynamic Dispatch

Recall the monomorphization process performed by the compiler when we use trait bounds on generic: the compiler generate nongeneric implementations of functions and methods for each concrete type that we use in place of a generic type parameter. The code that results from monomorphization is doing *static dispatch*, which is when the compiler knows what method you're calling at compile time. This is opposed to *dynamic dispatch*, which is when the compiler can't tell at compile time which method you're calling. In dynamic dispatch cases, the compiler emits code that at runtime will figure out which method to call.

When we use trait objects, Rust must use dynamic dispatch. The compiler doesn't know all the types that might be used with the code that's using trait objects, so it doesn't know which method implemented on which type to call. This lookup incurs a runtime cost that doesn't occur with static dispatch. Dynamic dispatch also prevents the compiler from choosing to inline a method's code, which in turn prevents some optimizations. However, we did get extra flexibility in the code, so it's a trade-off to consider.

