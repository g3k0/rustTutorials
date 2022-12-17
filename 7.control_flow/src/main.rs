fn main() {
    println!("Hello, world!");

    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // if the condition to check is not a bool, rust throws an error

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

    let condition = true;

    let number = if condition { 5 } else { "six" }; // returns an error because data types are different

    println!("The value of number is: {number}");


    // loops
    loop {
        println!("again!");
    } // infinite loop

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    // You can optionally specify a loop label 
    // Loop labels must begin with a single quote
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

    // while
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");

    // it is possible to use while loops with arrays
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    // As a more concise alternative, you can use a for loop
    for element in a {
        println!("the value is: {element}");
    }

    // Range, provided by the standard library, which generates all numbers in sequence starting from one number and ending before another number.
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");


}
