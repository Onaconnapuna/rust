// If expressions

fn if_expression() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}

// this errors out

// fn bool_error() {
//     let number = 3;

//     if number {
//         println!("number was three");
//     }
// }

fn bool_correct() {
    let number = 3;

    if number != 0 {
        println!("number was something other than zero");
    }
}

fn if_else() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

// ifs are expressions that return a value, I should avoid calling ifs 'statements'

// since ifs are expressions, we can do this 

fn if_condition_right_let() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}

// all potential return types of the if condition must be the same, considering that the if statement is an expression that returns something
// all contigencies or 'arms' of the if must return the same type.


// expected integer, found `&str`

// fn if_condition_right_let_error() {
//     let condition = true;

//     let number = if condition { 5 } else { "six" };

//     println!("The value of number is: {number}");
// }

// Loops

fn infinite_loop() {
    loop {
        println!("again!");
    }
}


// return the value from an if with break and the name of the value
// let result = loop  {   ...code... }
// interesting... we can explicity declare a variable to be the result of a loop
// and because the if is an expression... we omit the semi-colon

fn return_value_with_break() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

// notice the odd syntax 'counting_up: loop { } 
// this is called a loop label, and by saying break 'counting_up; we break the outer loop from the inner
// loop labels must begin with a single quote.

fn nested_loops() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

// much of this can be achieved... that being loop, if, else, and break with a while loop

fn while_loop() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn while_loop_with_collection() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}

// clean for loop in Rust

fn for_loop() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}

fn for_loop_range() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}