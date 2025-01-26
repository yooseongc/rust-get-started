

pub fn main() {

    // The String type is a growable, mutable, owned, UTF-8 encoded string type.
    // both String and string slices are UTF-8 encoded

    // String is actually implemented as a wrapper around a vector of bytes
    //    with some extra guarantees, restrictions, and capabilities

    // creates a new, empty mutable string called s
    let mut s = String::new();

    // from literal (initial data) with to_string
    let data = "initial contents";
    let s = data.to_string(); // from any type that implements the 'Display' trait
    let s = "initial contents".to_string();  // from literal directly

    // or from literal using 'from'
    let s = String::from("initial contents");

    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שלום");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    // Updating a String : concatenate String value 
    //    with '+' operator, format! macro, push_str()

    let mut s = String::from("foo");
    s.push_str("bar");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);  // takes a string slice
    println!("s2 is {s2}");

    let mut s = String::from("lo");
    s.push('l');  // push() adds the letter to a String

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;  // s1 has been movedhere and can no longer be used
    // + operator use the 'add' method : fn add(self, s: &str) -> String
    // compiler can 'coerce' the '&String' argument into a '&str' (deref coercion)
    //    which turns &s2 into &s2[..]

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;
    
    let s1 = String::from("tic");
    let s = format!("{s1}-{s2}-{s3}"); // format! macro doesn’t take ownership of any of its parameters.

    let s1 = String::from("hello");
    // let h = s1[0]; // <- string indices are ranges of `usize`
    //    the trait `SliceIndex<str>` is not implemented for `{integer}`

    // A String is a wrapper over a Vec<u8>, but it is encoded UTF-8...
    let hello = String::from("Hola");         // len will be 4
    // Each of these letters takes one byte when encoded in UTF-8.
    let hello = String::from("Здравствуйте"); // len will be 24
    // each Unicode scalar value in that string takes 2 bytes of storage.
    // UTF-8 => as bytes, scalar values, and grapheme clusters

    // Slicing Strings
    let hello = "Здравствуйте";
    let s = &hello[0..4];  // 4 bytes => 'Зд'
    // let s = &hello[0..1];  //  panick => byte index 1 is not a char boundary; it is inside 'З'

    // Iterating Over Strings
    for c in "Зд".chars() {
        println!("{c}");
    }

    for b in "Зд".bytes() {
        println!("{b}");
    }

    // Be sure to remember that valid Unicode scalar values may be made up of more than one byte.
    // Getting grapheme clusters from strings, as with the Devanagari script, is complex, 
    //     so this functionality is not provided by the standard library.
}
