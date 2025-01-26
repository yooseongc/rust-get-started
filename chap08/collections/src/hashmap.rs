

pub fn main() {
    // The type HashMap<K, V> stores a mapping of keys of type K to values of type V using a hashing function

    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // Like vectors, hash maps are homogeneous: 
    //     all of the keys must have the same type, and all of the values must have the same type.

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    // get method returns an Option<&V>
    // if there's no value for that key in the hash map, get() will return None
    // handles the Option by calling copied() to get an Option<i32> rather than an Option<&i32>
    // then unwrap_or() to set score to zero if scores doesn't have an entry for the key

    // print each pair in arbitary order
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name, field_value are invalid at this point (moved into the hash map with the call to insert)

    // Overwriting a Value
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    println!("{scores:?}");

    // Adding a Key and Value Only If a Key Isn't Present
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{scores:?}");

    // Updating a Value Based on the Old Value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");
    
}
