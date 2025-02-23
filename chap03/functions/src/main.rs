fn main() {
    println!("Hello, world!");

    another_function();
    another_function2(5);
    print_labeled_measurement(5, 'h');

    let y = 6;   // statement : do not return values
    let y = {
        let x = 3;
        x + 1
    };           // curly brackets is an expression
    // expressions do not include ending semicolons
    
    let x = five();
    println!("The value of x is: {x}");

    let x = plus_one(5);
    println!("The value of x is: {x}");
    

}

fn another_function() {
    println!("Another function.");
}

fn another_function2(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("the measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}




