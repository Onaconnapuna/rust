// Rust is a statically typed language, meaning the compiler 
// must know the types at compilation.
// Sometimes the compiler can infer, but when multiple types are possible
// we need type annotation

// SCALAR TYPES - represents a single value

// let guess: u32 = "42".parse().expect("Not a number!");

// if we remove : u32 the program wont compile due to type ambiguity

// Length	Signed	Unsigned
// 8-bit	i8	u8
// 16-bit	i16	u16
// 32-bit	i32	u32
// 64-bit	i64	u64
// 128-bit	i128|u128
// arch	    isize|usize

fn types() {

    //float
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    //booleans
    let t = true;

    let f: bool = false; // with explicit type annotation

    //characters
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
}

// COMPOUND TYPES - group multiple types into one type 
// two primary -- tuples and arrays

fn tuple_example() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
}

// arrays in rust have fixed length

fn array_example() {
    let a = [1, 2, 3, 4, 5];

    // initialize [ type, #ele ]
    let a_two: [i32; 5] = [1, 2, 3, 4, 5];

    // initialize [value, length 5 with value of value]
    let a = [3; 5];

}

use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];

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