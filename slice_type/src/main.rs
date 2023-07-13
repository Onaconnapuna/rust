// without slices 

// fn first_word(s: &String) -> ?

// this is not! ideal

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
//  let bytes = s.as_bytes();
//  converts our string to an array of bytes
//      for (i, &item) in bytes.iter().enumerate() 
// know that iter is a method that returns each element in a 
// collection and that enumerate wraps the result of iter and returns each element as part of a tuple instead
//  if item == b' ' {
//      return i;
//  }
// }
// s.len()
//  item==b' ' is byte literal syntax

// We have string slice type!

// let s = String::from("hello world");

// let hello = &s[0..5];
// let world = &s[6..11];

// [ start_position_inclusive..end_position_excluded ]

fn index_0() {
    let s = String::from("hello");

    let slice = &s[0..2];
    let slice = &s[..2];
}

fn index_last() {
    let s = String::from("hello");

    let len = s.len();

    let slice = &s[3..len];
    let slice = &s[3..];
}

fn first_word_new(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// let s = "Hello, world!";
// s is of type &str, which is just a slice reference to a part of memory, thats why they are immutable 

fn main() {
    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
}

// slices also work with arrays

fn slice_array() {
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
}