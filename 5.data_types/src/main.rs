/**
 * Keep in mind that Rust is a statically typed language, 
 * which means that it must know the types of all variables 
 * at compile time.
 */

fn main() {
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("The value of guess is {guess}");

    // A scalar type represents a single value
    // Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters

    // flaoting pint numbers
    let x = 2.0; // f64 64bit, the default in Rust
    let y: f32 = 3.0; // f32 32bit

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    // booleans
    let t = true;

    let f: bool = false; // with explicit type annotation

    // characters
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    /**
     * Note that we specify char literals with single quotes, 
     * as opposed to string literals, which use double quotes
     */

     /**
      * Compound types can group multiple values into one type.
      Rust has two primitive compound types: tuples and arrays.
      */

    /**
     * A tuple is a general way of grouping together a number 
     * of values with a variety of types into one compound type.
     * Tuples have a fixed length: once declared, they cannot 
     * grow or shrink in size.
     */

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup; // destructuring a tuple
    // how to access tuple elements
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    println!("The value of y is: {y}");

    // The tuple without any values has a special name, unit

    // array. every element of an array must have the same type
    let a = [1, 2, 3, 4, 5];
    /**
     * An array isn’t as flexible as the vector type, though.
     * A vector is a similar collection type provided by
     * the standard library that is allowed to grow
     * or shrink in size
     */
    let a: [i32; 5] = [1, 2, 3, 4, 5]; // the type declaration contains the array length even
    let first = a[0];
    let second = a[1];

}
