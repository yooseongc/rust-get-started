fn main() {
    
    // explicitly call panic! macro
    // panic!("crash and burn");

    // Attempting to access an element beyond the end of a vector, which will cause a call to panic!
    let v = vec![1, 2, 3];
    v[99];
}
