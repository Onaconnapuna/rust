// match example

// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter,
// }

// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => 1,
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter => 25,
//     }
// }

// can use curly brackets for each "arm" of the match condition, commas are optional as well

// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => {
//             println!("Lucky penny!");
//             1
//         }
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter => 25,
//     }
// }

// #[derive(Debug)] // so we can inspect the state in a minute
// enum UsState {
//     Alabama,
//     Alaska,
//     // --snip--
// }

// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter(UsState),
// }

// // I believe coin is ultimately being assinged a value here, and the coin: Coin parameter would look like value_in_cents(Coin::Quarter(state))
// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => 1,
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter(state) => {
//             println!("State quarter from {:?}", state)
//         },
//     }
// }

// fn main() {

//     { 
//         fn plus_one(x: Option<i32>) -> Option<i32> {
//             match x {
//                 None => None,
//                 Some(i) => Some(i + 1),
//             }
//         }
    
//         let five = Some(5);
//         let six = plus_one(five);
//         let none = plus_one(None);
//     }
// }

// matching with Option<T>
// Combining match and enums is useful in many situations. 
// You’ll see this pattern a lot in Rust code: match against an enum, bind a variable to the data inside, 
// and then execute code based on it. It’s a bit tricky at first, 
// but once you get used to it, you’ll wish you had it in all languages. It’s consistently a user favorite.

// match must cover all possibilites or else we wont compile

// fn plus_one(x: Option<i32>) -> Option<i32> {
//     match x {
//         Some(i) => Some(i + 1),
//     }
// }

// this wont compile becuause we did not cover the case for None

// enums are exhaustive, but there is a catch all branch

// let dice_roll = 9;
// match dice_roll {
//     3 => add_fancy_hat(),
//     7 => remove_fancy_hat(),
//     other => move_player(other),
// }

// fn dice_roll_game() {
//     fn add_fancy_hat() {}
//     fn remove_fancy_hat() {}
//     fn move_player(num_spaces: u8) {}
    
//     // catch all must be last, of course
    
//     let dice_roll = 9;
//     match dice_roll {
//         3 => add_fancy_hat(),
//         7 => remove_fancy_hat(),
//         _ => reroll(),
//     }
    
//     fn add_fancy_hat() {}
//     fn remove_fancy_hat() {}
//     fn reroll() {}
// }

// if we dont want to bind a value, we can use _

fn main() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
}

// Here, we’re telling Rust explicitly that we aren’t going to use any other value that doesn’t match a pattern in an earlier arm, 
// and we don’t want to run any code in this case.