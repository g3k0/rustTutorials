fn main() {
    println!("Hello, world!");

    // Rust code uses snake case as the conventional style for function and variable names

    another_function(5);
    print_labeled_measurement(5, 'h');

    /**
     * Statements are instructions that perform some action and do not return a value.
     * Expressions evaluate to a resultant value. Let’s look at some examples.
     */
    let x = (let y = 6); // this returns an error because let y = 6 returns nothing

    // this is an expression, it returns a value
    let y = {
        let x = 3;
        x + 1
    };

    /**
     * Calling a function is an expression. 
     * Calling a macro is an expression. 
     * A new scope block created with curly brackets
     * is an expression
     */

    /**
     * Expressions do not include ending semicolons
     * If you add a semicolon to the end of an expression,
     * you turn it into a statement, and it will then
     * not return a value
     */

     /**
      * Functions can return values to the code that calls them.
      We don’t name return values, but we must declare their
      type after an arrow (->).
      */
    fn five() -> i32 {
        5
    }
    let x = five();
    println!("The value of x is: {x}");

    // example with a parameter
    let x = plus_one(5);
    println!("The value of x is: {x}");

    fn plus_one(x: i32) -> i32 {
        x + 1
    }
}


// Rust doesn’t care where you define your functions
fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}
