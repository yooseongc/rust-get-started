fn main() {
    let mut s = String::from("hello world");

    let _word = first_word(&s);

    s.clear();

    let s2 = String::from("hello world");

    let _hello = &s2[0..5];
    let _world = &s2[6..11];

    let _first_word = first_word_2(&s2);

    main2()
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word_2(s: &String) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn first_word_3(s: &str) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main2() {

    let my_string = String::from("hello world");

    let _word = first_word_3(&my_string[0..6]);
    let _word = first_word_3(&my_string[..]);
    let _word = first_word_3(&my_string);

    let my_string_literal = "hello world";

    let _word = first_word_3(&my_string_literal[0..6]);
    let _word = first_word_3(&my_string_literal[..]);

    let _word = first_word_3(my_string_literal);

}