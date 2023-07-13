// structs 
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");
}

// build User suboptimal 

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

// build user with field init shorthand, 
fn build_user_better(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

// can create other instances from a another instance

fn user_two() {
    // --snip--

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };
}

// even better yet..

fn user_two_better() {
    // --snip--

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
}

// creating a user like this will make user 1 invalid, because string and username field were moved. active and sign_in have the 
// Copy trait

// Tuple structs have the added meaning the struct name provides 
// but don’t have names associated with their fields; rather, they just have the types of the fields

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn tuple_struct() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

// unit-like structs; structs without any fields
 
struct AlwaysEqual;

fn unit() {
    let subject = AlwaysEqual;
}

// struct User {
//     active: bool,
//     username: &str,
//     email: &str,
//     sign_in_count: u64,
// }

// fn main() {
//     let user1 = User {
//         active: true,
//         username: "someusername123",
//         email: "someone@example.com",
//         sign_in_count: 1,
//     };
// }

// compliler will say something like 'expected a lifetime parameter' will explore in chapter 10
