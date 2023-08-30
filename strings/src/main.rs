// The String type, which is provided by Rust’s 
// standard library rather than coded into the core language, 
// is a growable, mutable, owned, UTF-8 encoded string type

fn main () {
    let mut s = String::new();

    let data = "initial contents";

    let s = data.to_string();

    // the method also works on a literal directly:
    // let s = "initial contents".to_string();

    ownership()

}

fn to_string_func() {
    let data = "initial contents";
    
    let s = data.to_string();

    // the method also works on a literal directly:
    let s = "initial contents".to_string();
}


fn string_from() {
    // probaby better and more readable 
    let s = String::from("initial contents");
}

// push_str
fn strings_cont () {
    let mut s = String::from("foo");
    s.push_str("bar");
}

// the cooler thing about rust is...
fn strings_utf8() {
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");
}

fn ownership() {
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");
    // this somehow works, the book doesnt really explain why... we should lose ownership of the s2 variable; we didnt ?
}

fn concatenation() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used 
}
// we lose s1, because + uses the add operator, which looks like this fn add(self, s: &str) -> String {
// The reason we’re able to use &s2 in the call to add is that the compiler can coerce the &String argument into a &str. 
// When we call the add method, Rust uses a deref coercion, which here turns &s2 into &s2[..]


// not preferable way of cocatenating multiple strings
fn bad_concat() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    
    let s = s1 + "-" + &s2 + "-" + &s3;
}

fn better_concat() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");
}

fn cannot_idx_this_way() {
    let s1 = String::from("hello");
    let h = s1[0];
}

// A String is a wrapper over a Vec<u8>. 
// Let’s look at some of our properly encoded UTF-8 example strings from Listing 8-14. First, this one:

fn why() {
    // 4 bytes of memory
    let hello = String::from("Hola");

    //12? no. 24 because of the utf8 characters.
    let hello = String::from("Здравствуйте");
}

iterating_over_strings() {
    // gets the characters
    for c in "Зд".chars() {
        println!("{c}");
    }
    
    // gets bytes
    for b in "Зд".bytes() {
        println!("{b}");
    }
}