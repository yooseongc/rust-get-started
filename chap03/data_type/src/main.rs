use std::io;

fn main() {
    
    /*
     * Scalar Types
     * 
     *   - primary : integers, floating-point numbers, booleans, characters
     *   
     */

    // integer
    let fourty_two: u32 = "42".parse().expect("Not a number!");

    // floating-point
    let x = 2.0;      // f64
    let y: f32 = 3.0; // f32

    // numeric operations
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;  
    let truncated = -5 / 3;      // -1
    let remainder = 43 % 5;

    // boolean
    let t = true;
    let f: bool = false;

    // character -> 4 bytes unicode scalar value
    let c = 'z';
    let z: char = 'ℤ';
    let heart_eyed_cat = '😻';

    /*
     * Compound Types : group multiple values into one type
     *   - tuple
     *   - arrays
     */

    // tuple : a general way of grouping together a number of values with a variety of types into one compound type
    //         tuples have a fixed length: once declared, they cannot grow or shrink in size
    //         each position in the tuple has a type

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {y}");

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    // the tuple without any values has a special name 'unit'

    // array : every element of an array must have the same type, has a fixed length
    
    let months = ["January", "February", "March", "April", "May", "June", "July",
                  "August", "September", "October", "November", "December"];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let a = [3; 5];
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];

    println!("Please enter an array index.");
    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");

}
