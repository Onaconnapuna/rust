fn main() {
    println!("Hello, world!");

    another_function(5, 'h');
}

fn another_function(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// semi-colons turn the expression into a statement and do not return the values

fn expression() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}

fn five() -> i32 {
    5
    // notice no semi colon because we want to return this value
}

fn return_value() {
    let x = five();

    println!("The value of x is: {x}");
}

fn return_value_two() {
    let x = plus_one(5);

    println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
    // if we add a semi colon, this becomes a statement, meaning it will return and emptu tuple. 
}