// mutatbility and immutability 

// fn main() {
//     let mut x = 5;
//     println!("The value of x is: {x}");
//     x = 6;
//     println!("The value of x is: {x}");
// } 

// constant declaration ex.

// const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;


// shadowing

fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    // can change type with shadowing 
    let spaces = "   ";
    let spaces = spaces.len();

    // mut will throw an error, we need to use let

    // let mut spaces = "   ";
    // spaces = spaces.len();

    // mut can make a variable mutable, but not cannot change type
}