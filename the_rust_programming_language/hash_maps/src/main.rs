fn main() {
    adding_key_if_not_present()
}

fn hash_maps() {

    // creating a hash map
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
}

fn accessing_vals () {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
}

// .get() returns an Option<&V> 
// This program handles the Option by calling copied to get an Option<i32> rather than an Option<&i32>, 
// then unwrap_or to set score to zero if scores doesn't have an entry for the key.


// iterating over
fn iterating() {
    use std::collections::HashMap;
    
    let mut scores = HashMap::new();
    
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    
    for (key, value) in &scores {
        println!("{key}: {value}");
    }
}

// ownership 

fn ownership() {
    use std::collections::HashMap;

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!
}

fn inserting() {
    use std::collections::HashMap;

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!
}

fn adding_key_if_not_present() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);
    // he return value of the entry method is an enum called Entry that represents a value that might or might not exist. 
    // Let’s say we want to check whether the key for the Yellow team has a value associated with it. 
    // If it doesn’t, we want to insert the value 50, and the same for the Blue team.
    // Entry returns the value or not
}
// its semantic though hard to wrap my head around.. entry returns the value, or_insert, based on wheter or not entry returned the value
// inserts the key value pair

// entry does not overwrite the value of the Blue in this case, because the key already exists

fn update_val_basedon_val(){
    use std::collections::HashMap;

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}

// The or_insert method returns a mutable reference (&mut V) to the value for the specified key. 
// Here we store that mutable reference in the count variable, so in order to assign to that value, 
// we must first dereference count using the asterisk (*). The mutable reference goes out of scope at the end of the for loop, 
// so all of these changes are safe and allowed by the borrowing rules.