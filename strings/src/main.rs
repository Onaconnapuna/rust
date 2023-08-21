// The String type, which is provided by Rust’s 
// standard library rather than coded into the core language, 
// is a growable, mutable, owned, UTF-8 encoded string type

fn main () {
    let mut s = String::new();

    let data = "initial contents";

    let s = data.to_string();

    // the method also works on a literal directly:
    // let s = "initial contents".to_string();

}

// push_str
fn strings_cont () {
    let mut s = String::from("foo");
    s.push_str("bar");
}
