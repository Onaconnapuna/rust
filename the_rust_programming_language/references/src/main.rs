// instead of returning a tuple, we can just pass a reference

fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

// These ampersands represent references, and they allow you to refer to some value without taking ownership of it.

// s points to s1, s1 points to the heap

// fn calculate_length(s: &String) -> usize { // s is a reference to a String
    // s.len()
// } Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, it is not dropped.

  fn change_var() {
    let s = String::from("hello");

    // change(&s);
}

// fn change(some_string: &String) {
//     some_string.push_str(", world");
// }

// this does not work, references are immutable by default, just like variables, we must use mut

// this does work 
fn change_var_mut() {
    let mut s = String::from("hello");

    change_two(&mut s);
}

fn change_two(some_string: &mut String) {
    some_string.push_str(", world");
}

// if you have a mutable reference, you cannot have any more references to that value
// this does not work
// fn multiple_muts() {
//     let mut s = String::from("hello");

//     let r1 = &mut s;
//     let r2 = &mut s;

//     println!("{}, {}", r1, r2);
// }

// This design choice to prevent data races at compile time


// can have multiple mutable references as long as theyre not in the same 
fn multiple_ref_with_scope() {
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // r1 goes out omake a new ref scope here, so we can ference with no problems.

    let r2 = &mut s;
}

// fn multiple_refs_bad() {
//     let mut s = String::from("hello");

//     let r1 = &s; // no problem
//     let r2 = &s; // no problem
//     // let r3 = &mut s; // BIG PROBLEM

//     // println!("{}, {}, and {}", r1, r2, r3);
// }

// multiple immutable refs are fine, because any part of the program can read data; 

// a references scope starts from where its introduced and continues through the last time that reference is used

// this works 

fn reference_scope() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
}

// the compiler can tell that the reference is no longer being used at a point before the end of the scope.

// dangling poiters

// fn dangle_demo() {
//     let reference_to_nothing = dangle();
// }

// fn dangle() -> &String {
//     let s = String::from("hello");

//     &s
// }

// fn dangle() -> &String { // dangle returns a reference to a String

    // let s = String::from("hello"); // s is a new String

    // &s // we return a reference to the String, s
// } // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!

//   Because s is created inside dangle, when the code of dangle is finished, s will be deallocated.
//   But we tried to return a reference to it. That means this reference would be pointing to an invalid String

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}